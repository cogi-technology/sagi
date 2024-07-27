//Apache-2.0 License

use ethers::types::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};

use super::UserOperationSigned;

#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOperationRequest {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Bytes,
    pub call_data: Bytes,
    pub call_gas_limit: Option<U256>,           //
    pub verification_gas_limit: Option<U256>,   //
    pub pre_verification_gas: Option<U256>,     //
    pub max_fee_per_gas: Option<U256>,          //
    pub max_priority_fee_per_gas: Option<U256>, //
    pub paymaster_and_data: Bytes,
    pub signature: Option<Bytes>,
}

impl From<UserOperationRequest> for UserOperationSigned {
    fn from(user_operation: UserOperationRequest) -> Self {
        Self {
            sender: user_operation.sender,
            nonce: user_operation.nonce,
            init_code: user_operation.init_code,
            call_data: user_operation.call_data,
            call_gas_limit: {
                if let Some(call_gas_limit) = user_operation.call_gas_limit {
                    call_gas_limit
                } else {
                    U256::zero()
                }
            },
            verification_gas_limit: {
                if let Some(verification_gas_limit) = user_operation.verification_gas_limit {
                    verification_gas_limit
                } else {
                    U256::zero()
                }
            },
            pre_verification_gas: {
                if let Some(pre_verification_gas) = user_operation.pre_verification_gas {
                    pre_verification_gas
                } else {
                    U256::zero()
                }
            },
            max_fee_per_gas: {
                if let Some(max_fee_per_gas) = user_operation.max_fee_per_gas {
                    max_fee_per_gas
                } else {
                    U256::zero()
                }
            },
            max_priority_fee_per_gas: {
                if let Some(max_priority_fee_per_gas) = user_operation.max_priority_fee_per_gas {
                    max_priority_fee_per_gas
                } else {
                    U256::zero()
                }
            },
            paymaster_and_data: user_operation.paymaster_and_data,
            signature: {
                if let Some(signature) = user_operation.signature {
                    signature
                } else {
                    Bytes::default()
                }
            },
        }
    }
}

impl From<UserOperationSigned> for UserOperationRequest {
    fn from(user_operation: UserOperationSigned) -> Self {
        Self {
            sender: user_operation.sender,
            nonce: user_operation.nonce,
            init_code: user_operation.init_code,
            call_data: user_operation.call_data,
            call_gas_limit: Some(user_operation.call_gas_limit),
            verification_gas_limit: Some(user_operation.verification_gas_limit),
            pre_verification_gas: Some(user_operation.pre_verification_gas),
            max_fee_per_gas: Some(user_operation.max_fee_per_gas),
            max_priority_fee_per_gas: Some(user_operation.max_priority_fee_per_gas),
            paymaster_and_data: user_operation.paymaster_and_data,
            signature: Some(user_operation.signature),
        }
    }
}
