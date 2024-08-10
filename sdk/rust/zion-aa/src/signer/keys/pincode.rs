use {
    super::KeyBase,
    crate::{
        constants::OWNER_ROLE_WEIGHT,
        types::key::{KeyType, RoleWeight},
    },
    anyhow::Result,
    ethers::{
        abi::{AbiEncode, Token},
        signers::{LocalWallet, Signer},
        types::{Bytes, H256},
    },
    std::sync::Arc,
};

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
            Token::Address(self.inner.address()),
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::utils::make_pin_code_holder;

    use super::*;

    #[tokio::test]
    async fn test_generate_signature_is_ok() {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let pin_code = PINCode::new(Arc::new(wallet));

        let digest_hash =
            H256::from_str("0x140489a20d9fc3f204ce52b230c633b95ca7d1c68c38576fc432dde3c506ab1c")
                .unwrap();

        assert!(pin_code.generate_signature(digest_hash).await.is_ok());
    }

    #[test]
    fn test_serialize_is_ok() {
        let pin_code_holder = make_pin_code_holder(
            "123456".into(),
            "8b007c3425216674ebb4db21f7531a274fdf9e567173ef8d93d95a01375d26b0".into(),
        )
        .unwrap();

        let pin_code = PINCode::new(Arc::new(pin_code_holder));
        let serialized = pin_code.serialize().to_string();

        assert_eq!(
            serialized,
            "0x06646400e6ac0f8c4d83eb9af1ee42c40585a7f2015043a4".to_string()
        )
    }
}
