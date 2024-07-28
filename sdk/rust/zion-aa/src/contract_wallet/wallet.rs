use std::sync::Arc;

use ethers::types::Address;
use ethers_providers::Middleware;

use crate::{
    contracts::{Account, EntryPoint, Factory},
    signer::keys::{pincode::PINCode, KeyBase},
};

use super::operator::Operator;

pub struct ContractWallet<M, K> {
    contract: Arc<Account<M>>,
    pin_code: K,
    jwt_proof: K,
    operator: Arc<Operator<M>>
}

impl<M: Middleware + 'static, K: KeyBase + 'static> ContractWallet<M, K> {
    pub fn new(contract_wallet_address: Address, operator: Arc<Operator<M>>) -> Self {
        let operator = Arc::clone(&operator);
        let contract_wallet = Arc::new(Account::new(contract_wallet_address, operator.signer()));

        Self {
            contract: contract_wallet,
            pin_code: None,
            jwt_proof: None,
            operator,
        }
    }

    pub fn signer(&self) -> Arc<M> {
        self.operator.signer()
    }

    pub fn address(&self) -> Address {
        self.contract.address()
    }

    pub fn entry_point(&self) -> Arc<EntryPoint<M>> {
        self.operator.entry_point()
    }

    pub fn factory(&self) -> Arc<Factory<M>> {
        self.operator.factory()
    }

    pub fn contract(&self) -> Arc<Account<M>> {
        Arc::clone(&self.contract)
    }

    pub fn sub(&self) -> Option<&str> {
        self.jwt_proof.
    }

    pub fn salt(&self) -> Option<&str> {
        self.jwt_proof.as_ref().map(|jwt| jwt.inner.salt.as_str())
    }

    pub fn iss(&self) -> Option<&str> {
        self.jwt_proof.as_ref().map(|jwt| jwt.inner.payload.iss.as_str())
    }

    pub fn aud(&self) -> Option<&str> {
        self.jwt_proof.as_ref().map(|jwt| jwt.inner.payload.aud.as_str())
    }
}
