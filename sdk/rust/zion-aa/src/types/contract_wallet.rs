use ethers::types::Address;

#[derive(Debug, Clone)]
pub struct ContractWalletOperator {
    pub chain_id: u64,
    pub entrypoint_address: Address,
    pub factory_address: Address,
    pub verifying_paymaster_address: Option<Address>,
}
