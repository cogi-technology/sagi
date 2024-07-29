use std::sync::Arc;

use crate::{
    contracts::EntryPoint,
    signer::keys::KeyBase,
    types::user_operation::{request::UserOperationRequest, UserOperationSigned},
    utils::fill_user_op,
};
use anyhow::Result;
use ethers::{abi::Token, types::{Bytes, U256}};
use ethers_providers::Middleware;

pub async fn fill_and_sign<M: Middleware + 'static>(
    op: UserOperationRequest,
    signers: Vec<Arc<dyn KeyBase>>,
    entry_point: Arc<EntryPoint<M>>,
    chain_id: U256,
) -> Result<UserOperationSigned> {
    let mut op2 = fill_user_op(op, Arc::clone(&entry_point)).await?;
    let message = op2.hash(&entry_point.address(), chain_id);

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
