use {
    super::KeyBase,
    crate::{
        constants::OWNER_ROLE_WEIGHT,
        types::{
            jwt::JWTOptions,
            key::{KeyType, RoleWeight},
        },
        utils::groth16_export_solidity_call_data,
    },
    anyhow::Result,
    ethers::{
        signers::Signer,
        types::{Bytes, H256, U256},
    },
    ethers_core::abi::Token,
};

#[derive(Clone)]
pub struct KeyJWT<S> {
    pub inner: JWTOptions<S>,
    pub role_weight: RoleWeight,
}

impl<S: Signer + 'static> KeyJWT<S> {
    pub fn new(inner: JWTOptions<S>) -> Self {
        Self {
            inner,
            role_weight: OWNER_ROLE_WEIGHT,
        }
    }
}

#[async_trait::async_trait]
impl<S: Signer + 'static> KeyBase for KeyJWT<S> {
    async fn generate_signature(&self, digest_hash: H256) -> Result<Bytes> {
        let signature = self
            .inner
            .ephemeral_key_pair
            .sign_message(digest_hash)
            .await?;

        let call_data =
            groth16_export_solidity_call_data(self.inner.proof.clone(), vec!["0".into()]).await;

        let re = regex::Regex::new(r#"[\[\]"\s]"#).unwrap();
        let argv = re
            .replace_all(&call_data, "")
            .split(',')
            .map(|x| U256::from_str_radix(x, 16).unwrap())
            .collect::<Vec<U256>>();

        let a = Token::Array([Token::Uint(argv[0]), Token::Uint(argv[1])].into());
        let b1 = Token::Array([Token::Uint(argv[2]), Token::Uint(argv[3])].into());
        let b2 = Token::Array([Token::Uint(argv[4]), Token::Uint(argv[5])].into());
        let b = ethers::abi::encode_packed(&[b1, b2])?;
        let c = Token::Array([Token::Uint(argv[6]), Token::Uint(argv[7])].into());

        let abc = ethers::abi::encode_packed(&[a, Token::Bytes(b), c])?;
        let sig = ethers::abi::encode_packed(&[
            Token::Uint((KeyType::JWTZKProof as u8).into()),
            Token::Bytes(signature.to_vec()),
        ])?;
        let packed_deadline = ethers::abi::encode(&[Token::Uint(self.inner.deadline)]);

        let ret = ethers::abi::encode_packed(&[
            Token::Bytes(sig),
            Token::Bytes(packed_deadline),
            Token::Bytes(abc),
        ])?;

        Ok(ret.into())
    }

    fn serialize(&self) -> Bytes {
        ethers::abi::encode_packed(&[
            Token::Uint((KeyType::JWTZKProof as u8).into()),
            Token::Uint(self.weights().into()),
            Token::Bytes(self.get_hash().to_vec()),
        ])
        .unwrap()
        .into()
    }

    fn get_hash(&self) -> Bytes {
        let sub_in_hex = self.inner.payload.sub.as_bytes().to_vec();
        ethers::abi::encode_packed(&[
            Token::Uint((KeyType::JWTZKProof as u8).into()),
            Token::Bytes(sub_in_hex),
        ])
        .unwrap()
        .into()
    }

    fn role_weight(&self) -> RoleWeight {
        self.role_weight.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ethers::signers::LocalWallet;
    use jsonwebtoken::TokenData;

    use crate::types::{
        jwt::{JWTHeader, JWTPayload, ProofPoints},
        request::AuthorizationData,
    };

    use super::*;

    #[tokio::test]
    async fn test_generate_signature_is_ok() {
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
            authorization_data.ephemeral_key_pair,
            authorization_data.proof,
            authorization_data.salt,
        )
        .unwrap();
        let jwt_signer = KeyJWT::new(jwt_options);

        let digest_hash =
            H256::from_str("0x140489a20d9fc3f204ce52b230c633b95ca7d1c68c38576fc432dde3c506ab1c")
                .unwrap();

        let sig = jwt_signer
            .generate_signature(digest_hash)
            .await
            .unwrap()
            .to_string();

        assert_eq!(sig, "0x05999edae50c888dcea3ccbac47e56798b5e5c572b50b1af8da97b75f956638152112f3b0c1636670434c535c02167042af67954978c1b7a48db14ce4a632439bf1c0000000000000000000000000000000000000000000000000000000066aeea302707ab06a23fccb7a61ce837d279927261fc7298991ce4c2de52579179b40ca00071b3bc16fa972e97a5033b9dc05006b58142b16094746f1975be42203184860e52ac3105077c3dc6ebec0067e2014e1b1dd798067d18c96ed5d2b09732b79d23b63ea9ce7faf5bc10306f70ac2ae1fded6928709cce7e2a6b383c069e82495266240bd0f4a33026b3ebb6fb3906dff7a24dbc8684a914cf3a099237eb001851900fc374c7a06eeb7834dc3790f9101b218e97f103ee9f701f6e4bc99a961720f7fa3723c62fafb3fdc45b646a350db26bf2211cb73db903ba8ecd100aa7a012474c7bdeea27499775dc6fa9e8806d796b6d7c8a68dc2070f2b4effd1eb96ec".to_string());
    }

    #[ignore]
    #[test]
    fn test_serialize_is_ok() {
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
            authorization_data.ephemeral_key_pair,
            authorization_data.proof,
            authorization_data.salt,
        )
        .unwrap();
        let jwt_signer = KeyJWT::new(jwt_options);

        let serialized = jwt_signer.serialize().to_string();

        assert_eq!(
            serialized,
            "0x05646400099e4f96c8c97075f89aabf8209b1c5c966e7e5d958d496e6cb6c2db73810be6"
                .to_string()
        )
    }
}
