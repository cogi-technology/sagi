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
pub struct PayloadNftCallback {
    pub status: i32,
    pub namespace: String,
    pub param: ParamPayloadNftCallback,
}

impl Default for PayloadNftCallback {
    fn default() -> Self {
        Self {
            status: StatusEventCallback::Init.as_str(),
            namespace: "".into(),
            param: ParamPayloadNftCallback::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ParamPayloadNftCallback {
    pub owner: Address,
    pub txhash: String,
    pub cid: String,
    pub address: Address,
    pub token_id: U256,
}

impl Default for ParamPayloadNftCallback {
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

// Token ERC20

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PayloadTokenCallback {
    pub status: i32,
    pub namespace: String,
    pub param: ParamPayloadTokenCallback,
}

impl Default for PayloadTokenCallback {
    fn default() -> Self {
        Self {
            status: StatusEventCallback::Init.as_str(),
            namespace: "".into(),
            param: ParamPayloadTokenCallback::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ParamPayloadTokenCallback {
    pub owner: Address,
    pub txhash: String,
    pub address: Address,
    pub from: Address,
    pub to: Address,
    pub amount: U256,
}

impl Default for ParamPayloadTokenCallback {
    fn default() -> Self {
        Self {
            owner: Address::zero(),
            txhash: "".into(),
            address: Address::zero(),
            from: Address::zero(),
            to: Address::zero(),
            amount: U256::zero(),
        }
    }
}
