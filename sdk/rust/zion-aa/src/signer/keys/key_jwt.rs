use crate::{
    constants::OWNER_ROLE_WEIGHT,
    types::{
        jwt::JWTOptions,
        key::{KeyType, RoleWeight},
    },
    utils::groth16_export_solidity_call_data,
};
use anyhow::Result;
use ethers::{
    signers::Signer,
    types::{Bytes, H256, U256},
};
use ethers_core::abi::Token;

use super::KeyBase;

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
        println!("call_data: {:?}", call_data);
        let re = regex::Regex::new(r#"[\[\]"\s]"#).unwrap();
        let argv = re
            .replace_all(&call_data, "")
            .to_string()
            .split(',')
            .map(|x| U256::from_str_radix(x, 16).unwrap())
            .collect::<Vec<U256>>();

        println!("argv: {:?}", argv);

        let a = Token::Array([Token::Uint(argv[0]), Token::Uint(argv[1])].into());
        let b = Token::Array(
            [
                Token::Array([Token::Uint(argv[2]), Token::Uint(argv[3])].into()),
                Token::Array([Token::Uint(argv[4]), Token::Uint(argv[5])].into()),
            ]
            .into(),
        );
        let c = Token::Array([Token::Uint(argv[6]), Token::Uint(argv[7])].into());

        let sig = ethers::abi::encode_packed(&[
            Token::Uint((KeyType::JWTZKProof as u8).into()),
            Token::Bytes(signature.to_vec()),
        ])?;
        Ok(ethers::abi::encode(&[
            Token::Bytes(sig),
            Token::Uint(self.inner.deadline.clone()),
            a,
            b,
            c,
        ])
        .into())
    }

    fn serialize(&self) -> Bytes {
        ethers::abi::encode_packed(&[
            Token::Uint((KeyType::JWTZKProof as u8).into()),
            Token::Uint(self.weights().into()),
            Token::FixedBytes(self.get_hash().to_vec()),
        ])
        .unwrap()
        .into()
    }

    fn get_hash(&self) -> Bytes {
        let sub_in_hex = hex::decode(self.inner.payload.sub.clone()).unwrap();

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
