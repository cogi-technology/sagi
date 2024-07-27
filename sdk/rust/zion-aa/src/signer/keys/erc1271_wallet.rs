use crate::{
    types::key::{KeyType, RoleWeight},
    utils::serialize_role_weight,
};
use anyhow::Result;
use ethers::{
    abi::AbiEncode,
    signers::LocalWallet,
    types::{Address, Bytes, H256},
};
use ethers_core::abi::Token;
use std::sync::Arc;

use super::KeyBase;

#[derive(Clone)]
pub struct KeyERC1271Wallet {
    wallet_address: Address,
    inner: Arc<LocalWallet>,
    role_weight: RoleWeight,
}

impl KeyERC1271Wallet {
    pub fn new(wallet_address: Address, inner: Arc<LocalWallet>, role_weight: RoleWeight) -> Self {
        Self {
            wallet_address,
            inner,
            role_weight,
        }
    }
}

#[async_trait::async_trait]
impl KeyBase for KeyERC1271Wallet {
    async fn generate_signature(&self, digest_hash: H256) -> Result<Bytes> {
        let (signature, r) = self.inner.signer().sign_digest_recoverable(digest_hash)?;

        let mut _signature = signature.to_vec();
        _signature.push(r.to_byte());

        Ok(ethers::abi::encode(&[
            Token::Uint((KeyType::ERC1271Wallet as u8).into()),
            Token::Bytes(_signature),
        ])
        .into())
    }

    fn serialize(&self) -> Bytes {
        ethers::abi::encode(&[
            Token::Uint((KeyType::ERC1271Wallet as u8).into()),
            Token::Address(self.wallet_address),
            Token::Bytes(self.serialize_role_weight().to_vec()),
        ])
        .into()
    }

    fn get_hash(&self) -> Bytes {
        ethers::abi::encode(&[
            Token::Uint((KeyType::ERC1271Wallet as u8).into()),
            Token::FixedBytes(self.wallet_address.encode()),
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
