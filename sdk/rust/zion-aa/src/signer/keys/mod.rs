pub mod pincode;
pub mod secp256k1;
pub mod erc1271_wallet;

use anyhow::Result;
use ethers::types::Bytes;

#[async_trait::async_trait]
pub trait KeyBase {
    async fn generate_signature(&self, digest_hash: String) -> Result<Bytes>;
    fn serialize(&self) -> Bytes;
    fn get_hash(&self) -> Bytes;
    fn serialize_role_weight(&self) -> Bytes;
    fn weights(&self) -> usize;
}
