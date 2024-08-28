use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Config {
    #[serde(rename = "zion-rpc")]
    pub zion_rpc: String,

    #[serde(rename = "torii-rpc")]
    pub torii_rpc: String,

    #[serde(rename = "chain-id")]
    pub chain_id: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            zion_rpc: "https://devnet-rpc.zionx.network".into(),
            torii_rpc: "https://torii.zionx.network".into(),
            chain_id: 176923,
        }
    }
}
