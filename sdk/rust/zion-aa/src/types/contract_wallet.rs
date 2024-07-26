use ethers::types::Address;

pub struct ContractWalletOperator {
    pub chain_id: u64,
    pub entrypoint_address: Address,
    pub factory_address: Address,
    pub verifying_paymaster_address: Option<Address>,
}
