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
pub struct PINCode {
    inner: Arc<LocalWallet>,
    role_weight: RoleWeight,
}

impl PINCode {
    pub fn new(inner: Arc<LocalWallet>) -> Self {
        Self {
            inner,
            role_weight: OWNER_ROLE_WEIGHT,
        }
    }
}

#[async_trait::async_trait]
impl KeyBase for PINCode {
    async fn generate_signature(&self, digest_hash: H256) -> Result<Bytes> {
        let signature = self.inner.sign_message(digest_hash).await?;

        ethers::abi::encode_packed(&[
            Token::Uint((KeyType::PINCode as u8).into()),
            Token::Bytes(signature.into()),
        ])
        .map(|ok| ok.into())
        .map_err(|e| e.into())
    }

    fn serialize(&self) -> Bytes {
        ethers::abi::encode_packed(&[
            Token::Uint((KeyType::PINCode as u8).into()),
            Token::Uint(self.weights().into()),
            Token::FixedBytes(self.inner.address().encode()),
        ])
        .unwrap()
        .into()
    }

    fn get_hash(&self) -> Bytes {
        ethers::abi::encode_packed(&[
            Token::Uint((KeyType::PINCode as u8).into()),
            Token::FixedBytes(self.inner.address().encode()),
        ])
        .unwrap()
        .into()
    }

    fn role_weight(&self) -> RoleWeight {
        self.role_weight.clone()
    }
}
