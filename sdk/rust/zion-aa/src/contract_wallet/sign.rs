use super::Client;
use crate::{
    contracts::EntryPoint,
    signer::keys::KeyBase,
    types::user_operation::{request::UserOperationRequest, UserOperationSigned},
    utils::fill_user_op,
};
use anyhow::Result;
use ethers::{abi::Token, types::Bytes};

pub async fn fill_n_sign<S: KeyBase>(
    op: UserOperationRequest,
    signers: Vec<S>,
    entry_point: EntryPoint<Client>,
    chain_id: u64,
) -> Result<UserOperationSigned> {
    let mut op2 = fill_user_op(op, &entry_point).await?;
    let message = op2.hash(&entry_point.address(), chain_id);

    let mut sig = Bytes::new();
    for signer in signers {
        let new_sig = signer.generate_signature(message.into()).await?;
        sig = ethers::abi::encode(&[Token::Bytes(sig.to_vec()), Token::Bytes(new_sig.to_vec())])
            .into();
    }
    op2.signature = sig;

    Ok(op2)
}
