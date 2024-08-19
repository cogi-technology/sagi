use {
    crate::{
        signer::keys::KeyBase,
        types::user_operation::{request::UserOperationRequest, UserOperationSigned},
        utils::fill_user_op,
    },
    anyhow::Result,
    ethers::{
        abi::Token,
        providers::Middleware,
        types::{Address, Bytes, U256},
    },
    std::sync::Arc,
};

pub async fn fill_and_sign<M: Middleware + 'static>(
    op: UserOperationRequest,
    signers: Vec<Arc<dyn KeyBase + Send + Sync>>,
    entry_point_address: Address,
    entry_point_provider: Arc<M>,
    chain_id: U256,
) -> Result<UserOperationSigned> {
    let mut op2 = fill_user_op(op, Arc::clone(&entry_point_provider), entry_point_address).await?;

    let message = op2.hash(entry_point_address, chain_id)?;

    let mut sig = Bytes::new();
    for signer in signers {
        let new_sig = signer.generate_signature(message.into()).await?;
        sig = ethers::abi::encode_packed(&[
            Token::Bytes(sig.to_vec()),
            Token::Bytes(new_sig.to_vec()),
        ])
        .map(|ok| ok.into())?;
    }
    op2.mut_inner().signature = sig;

    Ok(op2)
}

#[cfg(test)]
mod test {
    use {
        super::*,
        crate::{
            signer::keys::{key_jwt::KeyJWT, pincode::PINCode, KeyBase},
            types::{
                jwt::{JWTHeader, JWTOptions, JWTPayload, ProofPoints},
                request::AuthorizationData,
                user_operation::UserOperationSigned,
            },
        },
        ethers::signers::LocalWallet,
        ethers_providers::{Http, Provider, ProviderExt},
        jsonwebtoken::TokenData,
    };

    #[tokio::test]
    async fn test_fill_and_sign() -> Result<()> {
        let authorization_data = AuthorizationData {
            salt: "8b007c3425216674ebb4db21f7531a274fdf9e567173ef8d93d95a01375d26b0".into(),
            proof: ProofPoints {
                pi_a: [
                    "17653749401672655941406011998008240109544620681447084170066298566525523856544".into(),
                    "200894201805963756131655553149559193873493052596125531325963901192664876166".into(),
                    "1".into(),
                ].into(),
                pi_b: [
                    [
                        "16152948352455335947987820179503103210826678292813482609191298065357366371477".into(),
                        "6478449761416232359021492072256236084350879056191463115621182278503322793885".into(),
                    ].into(),
                    [
                        "11309561945512050820579571164917365221398569177259775336899426456108383428978".into(),
                        "17361486067318727288401699964708457356740602553723465995615391323950079279493".into(),
                    ].into(),
                    [
                        "1".into(),
                        "0".into(),
                    ].into(),
                ].into(),
                pi_c: [
                    "7010210370423753225924484123467277614275751062858768542215873526511920904705".into(),
                    "16489595376595969697160625408276638505571216679970799399856631931671484798700".into(),
                    "1".into(),
                ].into(),
                protocol: Some(
                    "groth16".into(),
                ),
            },
            ephemeral_key_pair: "c03040d07874938f47c8e2bb99eadc161c0385af8e1239e4e55be3c18b9ea97e".into(),
            beneficiaries: [
                "0xfe39693d77c7c83e26ff7df39c13fc36f9cc88f5".into(),
            ].into(),
        };

        let token_data = TokenData {
            header: JWTHeader {
                typ: Some("JWT".into()),
                alg: jsonwebtoken::Algorithm::RS256,
                kid: Some("iGKUAONmCDAQNhAuB4qa9KBj".into()),
                ..Default::default()
            },
            claims: JWTPayload {
                iat: Some(1722652848),
                exp: 1722739248,
                nbf: 1722652847,
                iss: "https://id.telegram.org".into(),
                sub: "5740847399".into(),
                aud: "7109740482".into(),
                at_hash: None,
            },
        };

        let jwt_options = JWTOptions::<LocalWallet>::try_init(
            token_data,
            authorization_data.ephemeral_key_pair.clone(),
            authorization_data.proof,
            authorization_data.salt,
        )
        .unwrap();

        let request_op = UserOperationSigned::default();
        let wallet = authorization_data
            .ephemeral_key_pair
            .parse::<LocalWallet>()?;
        let provider = Provider::<Http>::connect("https://devnet-rpc.zionx.network").await;

        let jwt_signer = KeyJWT::new(jwt_options);
        let pin_code_signer = PINCode::new(Arc::new(wallet));

        let signers: Vec<Arc<dyn KeyBase + Send + Sync>> =
            vec![Arc::new(jwt_signer), Arc::new(pin_code_signer)];

        let result = fill_and_sign(
            request_op.into(),
            signers,
            "0xBDFa286897F86CD02b7916BC1E9aAdc1f09da842".parse()?,
            Arc::new(provider),
            176923.into(),
        )
        .await?;

        assert!(!result.into_inner().signature.is_empty());
        Ok(())
    }
}
