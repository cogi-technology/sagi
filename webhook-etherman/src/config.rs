use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Config {
    #[serde(rename = "ethereum-rpc")]
    pub ethereum_rpc: String,

    #[serde(rename = "chain-id")]
    pub chain_id: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ethereum_rpc: "https://devnet-rpc.zionx.network".into(),
            chain_id: 176923,
        }
    }
}
