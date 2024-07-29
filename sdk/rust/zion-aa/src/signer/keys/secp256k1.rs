use crate::{
    constants::OWNER_ROLE_WEIGHT,
    types::key::{KeyType, RoleWeight},
};
use anyhow::Result;
use ethers::{
    abi::AbiEncode,
    signers::{LocalWallet, Signer},
    types::{Bytes, H256},
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
    async fn generate_signature(&self, digest_hash: H256) -> Result<Bytes> {
        let signature = self.inner.sign_message(digest_hash).await?;

        ethers::abi::encode_packed(&[
            Token::Uint((KeyType::Secp256k1 as u8).into()),
            Token::Bytes(signature.into()),
        ])
        .map(|ok| ok.into())
        .map_err(|e| e.into())
    }

    fn serialize(&self) -> Bytes {
        ethers::abi::encode_packed(&[
            Token::Uint((KeyType::Secp256k1 as u8).into()),
            Token::Uint(self.weights().into()),
            Token::FixedBytes(self.inner.address().encode()),
        ])
        .unwrap()
        .into()
    }

    fn get_hash(&self) -> Bytes {
        ethers::abi::encode_packed(&[
            Token::Uint((KeyType::Secp256k1 as u8).into()),
            Token::FixedBytes(self.inner.address().encode()),
        ])
        .unwrap()
        .into()
    }

    fn role_weight(&self) -> RoleWeight {
        self.role_weight.clone()
    }
}
