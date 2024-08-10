pub mod key_jwt;
pub mod pincode;
pub mod secp256k1;
// pub mod erc1271_wallet;

use {
    crate::{types::key::RoleWeight, utils::serialize_role_weight},
    anyhow::Result,
    ethers::{
        abi::Token,
        types::{Bytes, H256},
    },
};

#[async_trait::async_trait]
pub trait KeyBase {
    async fn generate_signature(&self, digest_hash: H256) -> Result<Bytes>;
    fn serialize(&self) -> Bytes;
    fn get_hash(&self) -> Bytes;
    fn role_weight(&self) -> RoleWeight;
    fn serialize_role_weight(&self) -> Bytes {
        ethers::abi::encode_packed(&[
            Token::Uint(<Self>::role_weight(self).owner_weight.into()),
            Token::Uint(<Self>::role_weight(self).assets_op_weight.into()),
            Token::Uint(<Self>::role_weight(self).guardian_weight.into()),
        ])
        .unwrap()
        .into()
    }
    fn weights(&self) -> usize {
        serialize_role_weight(&self.role_weight())
    }
}
