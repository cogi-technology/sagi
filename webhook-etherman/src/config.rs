use {
    ethers::types::H160,
    serde::{Deserialize, Serialize},
    std::str::FromStr,
};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Config {
    #[serde(rename = "ethereum-rpc")]
    pub ethereum_rpc: String,

    #[serde(rename = "chain-id")]
    pub chain_id: u64,

    #[serde(rename = "operator-keystore")]
    pub operator_keystore: std::path::PathBuf,

    #[serde(rename = "contract-address")]
    pub contract_address: H160,

    #[serde(rename = "start-block-number")]
    pub start_block_number: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ethereum_rpc: "https://devnet-rpc.zionx.network".into(),
            chain_id: 176923,
            operator_keystore: "openapi-server/dist/operator-develop.keystore".into(),
            contract_address: H160::from_str("0xeA3b8eA006676E1cEf0019659a57e0C126c283c6").unwrap(),
            start_block_number: 614,
        }
    }
}
