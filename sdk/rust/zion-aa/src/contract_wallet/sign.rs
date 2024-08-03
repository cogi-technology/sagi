use std::sync::Arc;

use crate::{
    signer::keys::KeyBase,
    types::user_operation::{request::UserOperationRequest, UserOperationSigned},
    utils::fill_user_op,
};
use anyhow::Result;
use ethers::{
    abi::Token,
    types::{Address, Bytes, U256},
};
use ethers_providers::Middleware;

pub async fn fill_and_sign<M: Middleware + 'static>(
    op: UserOperationRequest,
    signers: Vec<Arc<dyn KeyBase + Send + Sync>>,
    entry_point_address: Address,
    entry_point_provider: Arc<M>,
    chain_id: U256,
) -> Result<UserOperationSigned> {
    let mut op2 = fill_user_op(op, Arc::clone(&entry_point_provider)).await?;

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
            contract_wallet::client::{Client, ClientMethods},
            signer::keys::{key_jwt::KeyJWT, KeyBase},
            types::{jwt::JWTOptions, user_operation::UserOperationSigned},
        },
        ethers::{abi::Address, types::U256},
    };

    #[tokio::test]
    async fn test_fill_and_sign() -> Result<()> {
        let op = UserOperationSigned::default();
        let chain_id = U256::from(1);
        let random_address = Address::random();
        let random_wallet = Arc::new(Client::random_wallet("http://localhost:8545", 1337).await?);
        let signer = Arc::new(random_wallet.signer().clone());
        let provider = Arc::new(random_wallet.provider().clone());

        let a = provider.get_accounts().await?;
        println!("{:#?}", a);

        let jwt_options = JWTOptions {
            header: Default::default(),
            payload: Default::default(),
            proof: Default::default(),
            ephemeral_key_pair: signer,
            deadline: Default::default(),
            salt: Default::default(),
        };

        let mock_signer = KeyJWT::new(jwt_options);

        let signers: Vec<Arc<dyn KeyBase + Send + Sync>> = vec![Arc::new(mock_signer)];

        let result = fill_and_sign(op.into(), signers, random_address, provider, chain_id).await?;

        assert!(!result.into_inner().signature.is_empty());
        Ok(())
    }
}
