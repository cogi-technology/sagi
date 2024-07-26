use crate::{
    constants::OWNER_ROLE_WEIGHT,
    types::key::{KeyType, RoleWeight},
    utils::serialize_role_weight,
};
use anyhow::Result;
use ethers::{
    abi::AbiEncode,
    signers::{LocalWallet, Signer},
    types::Bytes,
};
use ethers_core::abi::Token;
use std::sync::Arc;

use super::KeyBase;

#[derive(Clone)]
pub struct KeySecp256k1 {
    inner: Arc<LocalWallet>,
    role_weight: RoleWeight,
}

impl KeySecp256k1 {
    pub fn new(inner: Arc<LocalWallet>) -> Self {
        Self {
            inner,
            role_weight: OWNER_ROLE_WEIGHT,
        }
    }
}

#[async_trait::async_trait]
impl KeyBase for KeySecp256k1 {
    async fn generate_signature(&self, digest_hash: String) -> Result<Bytes> {
        let signature = self.inner.sign_message(digest_hash.into_bytes()).await?;

        Ok(ethers::abi::encode(&[
            Token::Uint((KeyType::Secp256k1 as u8).into()),
            Token::Bytes(signature.into()),
        ])
        .into())
    }

    fn serialize(&self) -> Bytes {
        ethers::abi::encode(&[
            Token::Uint((KeyType::Secp256k1 as u8).into()),
            Token::Uint(self.weights().into()),
            Token::FixedBytes(self.inner.address().encode()),
        ])
        .into()
    }

    fn get_hash(&self) -> Bytes {
        ethers::abi::encode(&[
            Token::Uint((KeyType::Secp256k1 as u8).into()),
            Token::FixedBytes(self.inner.address().encode()),
        ])
        .into()
    }

    fn serialize_role_weight(&self) -> Bytes {
        ethers::abi::encode(&[
            Token::Uint(self.role_weight.owner_weight.into()),
            Token::Uint(self.role_weight.assets_op_weight.into()),
            Token::Uint(self.role_weight.guardian_weight.into()),
        ])
        .into()
    }

    fn weights(&self) -> usize {
        serialize_role_weight(&self.role_weight)
    }
}
