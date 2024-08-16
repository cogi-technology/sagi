use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

pub enum StatusEventCallback {
    TxSuccess,
    Burned,
    Transfer,
    Init,
}

impl StatusEventCallback {
    pub fn as_str(&self) -> i32 {
        match self {
            StatusEventCallback::TxSuccess => 3,
            StatusEventCallback::Burned => -50,
            StatusEventCallback::Transfer => 50,
            StatusEventCallback::Init => 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PayloadCallback {
    pub status: i32,
    pub namespace: String,
    pub param: ParamPayloadCallback,
}

impl Default for PayloadCallback {
    fn default() -> Self {
        Self {
            status: StatusEventCallback::Init.as_str(),
            namespace: "".into(),
            param: ParamPayloadCallback::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ParamPayloadCallback {
    pub owner: Address,
    pub txhash: String,
    pub cid: String,
    pub address: Address,
    pub token_id: U256,
}

impl Default for ParamPayloadCallback {
    fn default() -> Self {
        Self {
            owner: Address::zero(),
            txhash: "".into(),
            cid: "".into(),
            address: Address::zero(),
            token_id: U256::zero(),
        }
    }
}
