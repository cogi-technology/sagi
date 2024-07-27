//Apache-2.0 License

pub mod hash;
pub mod request;

use std::ops::Deref;

use ethers::{
    abi::AbiEncode,
    types::{Address, Bytes, H256, U256},
    utils::keccak256,
};
use ethers_contract::{EthAbiCodec, EthAbiType};
use hash::UserOperationHash;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Serialize, Deserialize, EthAbiCodec, EthAbiType,
)]
#[serde(rename_all = "camelCase")]
pub struct UserOperationSigned {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: Bytes,
    pub call_data: Bytes,
    pub call_gas_limit: U256,           //
    pub verification_gas_limit: U256,   //
    pub pre_verification_gas: U256,     //
    pub max_fee_per_gas: U256,          //
    pub max_priority_fee_per_gas: U256, //
    pub paymaster_and_data: Bytes,
    pub signature: Bytes,
}

impl Default for UserOperationSigned {
    fn default() -> Self {
        Self {
            sender: Address::zero(),
            nonce: U256::zero(),
            init_code: Bytes::new(),
            call_data: Bytes::new(),
            call_gas_limit: U256::zero(),
            verification_gas_limit: U256::from(700_000),
            pre_verification_gas: U256::from(21_000),
            max_fee_per_gas: U256::from(2_000_000),
            max_priority_fee_per_gas: U256::from(2_000_000),
            paymaster_and_data: Bytes::new(),
            signature: Bytes::new(),
        }
    }
}

#[derive(EthAbiCodec, EthAbiType)]
struct UserOperationNoSignature {
    pub sender: Address,
    pub nonce: U256,
    pub init_code: H256,
    pub call_data: H256,
    pub call_gas_limit: U256,
    pub verification_gas_limit: U256,
    pub pre_verification_gas: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub paymaster_and_data: H256,
}

impl From<UserOperationSigned> for UserOperationNoSignature {
    fn from(value: UserOperationSigned) -> Self {
        Self {
            sender: value.sender,
            nonce: value.nonce,
            init_code: keccak256(value.init_code.deref()).into(),
            call_data: keccak256(value.call_data.deref()).into(),
            call_gas_limit: value.call_gas_limit,
            verification_gas_limit: value.verification_gas_limit,
            pre_verification_gas: value.pre_verification_gas,
            max_fee_per_gas: value.max_fee_per_gas,
            max_priority_fee_per_gas: value.max_priority_fee_per_gas,
            paymaster_and_data: keccak256(value.paymaster_and_data.deref()).into(),
        }
    }
}

impl UserOperationSigned {
    /// Packs the user operation into bytes
    pub fn pack(&self) -> Bytes {
        self.clone().encode().into()
    }

    /// Packs the user operation without signature to bytes (used for calculating the hash)
    pub fn pack_without_signature(&self) -> Bytes {
        let user_operation_packed = UserOperationNoSignature::from(self.clone());
        user_operation_packed.encode().into()
    }

    /// Calculates the hash of the user operation
    pub fn hash(&self, entry_point: &Address, chain_id: u64) -> UserOperationHash {
        H256::from_slice(
            keccak256(
                [
                    keccak256(self.pack_without_signature().deref()).to_vec(),
                    entry_point.encode(),
                    U256::from(chain_id).encode(),
                ]
                .concat(),
            )
            .as_slice(),
        )
        .into()
    }
}
