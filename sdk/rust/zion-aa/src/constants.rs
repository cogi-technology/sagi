use crate::types::{
    contract_wallet::ContractWalletOperator, key::RoleWeight, user_operation::UserOperationSigned,
};
use ethers::{
    abi::Address,
    types::{Bytes, U256},
};
use once_cell::sync::Lazy;

pub const OWNER_ROLE_WEIGHT: RoleWeight = RoleWeight {
    owner_weight: 100,
    assets_op_weight: 100,
    guardian_weight: 0,
};

pub const GUARDIAN_ROLE_WEIGHT: RoleWeight = RoleWeight {
    owner_weight: 0,
    assets_op_weight: 0,
    guardian_weight: 100,
};

pub static DEFAULTS_FOR_USER_OP: Lazy<UserOperationSigned> = Lazy::new(|| UserOperationSigned {
    sender: Address::zero(),
    nonce: U256::zero(),
    init_code: Bytes::new(),
    call_data: Bytes::new(),
    call_gas_limit: U256::zero(),
    verification_gas_limit: U256([700_000, 0, 0, 0]),
    pre_verification_gas: U256([21_000, 0, 0, 0]),
    max_fee_per_gas: U256([2_000_000, 0, 0, 0]),
    max_priority_fee_per_gas: U256([2_000_000, 0, 0, 0]),
    paymaster_and_data: Bytes::new(),
    signature: Bytes::new(),
});

pub static NEMOTESTNET: Lazy<ContractWalletOperator> = Lazy::new(|| ContractWalletOperator {
    chain_id: 25555,
    entrypoint_address: "0x1c753dD9955782aC974798A6f65dfFe03f217841"
        .parse::<Address>()
        .unwrap(),
    factory_address: "0xA70f2726eaB0E94d9c3EFbd525021e30eB6f8DE3"
        .parse::<Address>()
        .unwrap(),
    verifying_paymaster_address: None,
});

pub static COGITESTNET: Lazy<ContractWalletOperator> = Lazy::new(|| ContractWalletOperator {
    chain_id: 5555,
    entrypoint_address: "0x997BA705FedF1DeAB2a37864EEbB850232cE56B1"
        .parse()
        .unwrap(),
    factory_address: "0x592775270DabDE18AFa6122a84E6112dfEE61042"
        .parse()
        .unwrap(),
    verifying_paymaster_address: None,
});

pub static LOCALHOST: Lazy<ContractWalletOperator> = Lazy::new(|| ContractWalletOperator {
    chain_id: 15555,
    entrypoint_address: "0x1c753dD9955782aC974798A6f65dfFe03f217841"
        .parse()
        .unwrap(),
    factory_address: "0xea6ed16F1274aDf30181307a7e6284073fF84FDB"
        .parse()
        .unwrap(),
    verifying_paymaster_address: None,
});

pub static ZIONTESTNET: Lazy<ContractWalletOperator> = Lazy::new(|| ContractWalletOperator {
    chain_id: 176923,
    entrypoint_address: "0xBDFa286897F86CD02b7916BC1E9aAdc1f09da842"
        .parse()
        .unwrap(),
    factory_address: "0xEfE40749F5A7476045B045BE499706B9A06d55D7"
        .parse()
        .unwrap(),
    verifying_paymaster_address: None,
});
