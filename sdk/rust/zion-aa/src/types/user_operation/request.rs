//Apache-2.0 License

use ethers::types::{Address, Bytes, U256};
use serde::{Deserialize, Serialize};

use crate::contracts::entry_point::UserOperation;

use super::UserOperationSigned;

#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserOperationRequest {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Option<Bytes>,
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
    fn from(uo_request: UserOperationRequest) -> Self {
        UserOperationSigned(UserOperation {
            sender: uo_request.sender,
            nonce: uo_request.nonce,
            init_code: {
                if let Some(init_code) = uo_request.init_code {
                    init_code
                } else {
                    Bytes::default()
                }
            },
            call_data: uo_request.call_data,
            call_gas_limit: {
                if let Some(call_gas_limit) = uo_request.call_gas_limit {
                    call_gas_limit
                } else {
                    U256::zero()
                }
            },
            verification_gas_limit: {
                if let Some(verification_gas_limit) = uo_request.verification_gas_limit {
                    verification_gas_limit
                } else {
                    U256::zero()
                }
            },
            pre_verification_gas: {
                if let Some(pre_verification_gas) = uo_request.pre_verification_gas {
                    pre_verification_gas
                } else {
                    U256::zero()
                }
            },
            max_fee_per_gas: {
                if let Some(max_fee_per_gas) = uo_request.max_fee_per_gas {
                    max_fee_per_gas
                } else {
                    U256::zero()
                }
            },
            max_priority_fee_per_gas: {
                if let Some(max_priority_fee_per_gas) = uo_request.max_priority_fee_per_gas {
                    max_priority_fee_per_gas
                } else {
                    U256::zero()
                }
            },
            paymaster_and_data: uo_request.paymaster_and_data,
            signature: {
                if let Some(signature) = uo_request.signature {
                    signature
                } else {
                    Bytes::default()
                }
            },
        })
    }
}

impl From<UserOperationSigned> for UserOperationRequest {
    fn from(uo_signed: UserOperationSigned) -> Self {
        Self {
            sender: uo_signed.0.sender,
            nonce: uo_signed.0.nonce,
            init_code: Some(uo_signed.0.init_code),
            call_data: uo_signed.0.call_data,
            call_gas_limit: Some(uo_signed.0.call_gas_limit),
            verification_gas_limit: Some(uo_signed.0.verification_gas_limit),
            pre_verification_gas: Some(uo_signed.0.pre_verification_gas),
            max_fee_per_gas: Some(uo_signed.0.max_fee_per_gas),
            max_priority_fee_per_gas: Some(uo_signed.0.max_priority_fee_per_gas),
            paymaster_and_data: uo_signed.0.paymaster_and_data,
            signature: Some(uo_signed.0.signature),
        }
    }
}
