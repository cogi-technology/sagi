use super::client::{Client, ClientMethods};
use crate::{
    contracts::{EntryPoint, Factory, NemoAccountCreatedFilter},
    types::contract_wallet::ContractWalletOperator,
    utils::get_provider_hashed,
};
use anyhow::{anyhow, Error, Result};
use ethers::{
    abi::Token,
    types::{Address, Bytes},
};
use ethers_contract::EthLogDecode;
use ethers_providers::Middleware;
use rand::Rng;
use std::sync::Arc;

#[derive(Clone)]
pub struct Operator<M> {
    factory: Arc<Factory<M>>,
    entry_point: Arc<EntryPoint<M>>,
    beneficiaries: Vec<Address>,
}

impl<M: Middleware + 'static> Operator<M> {
    pub fn new(
        operator: ContractWalletOperator,
        signer: Arc<M>,
        beneficiaries: Vec<Address>,
    ) -> Self {
        let factory = Arc::new(Factory::new(operator.factory_address, Arc::clone(&signer)));
        let entry_point = Arc::new(EntryPoint::new(
            operator.entrypoint_address,
            Arc::clone(&signer),
        ));
        Self {
            factory,
            entry_point,
            beneficiaries,
        }
    }

    pub async fn get_ephemeral_key_pair(
        rpc_endpoint: &str,
        chain_id: u64,
        private_key: Option<&str>,
    ) -> Result<Client> {
        if rpc_endpoint.is_empty() {
            panic!("Endpoint is required");
        }
        if let Some(pk) = private_key {
            Client::try_new(rpc_endpoint, chain_id, pk).await
        } else {
            Client::random_wallet(rpc_endpoint, chain_id).await
        }
    }

    pub fn get_init_code(
        &self,
        sub: String,
        salt: [u8; 32],
        iss: String,
        aud: String,
    ) -> Result<Bytes> {
        let provider = get_provider_hashed(iss, aud);
        let sub_in_hex = hex::decode(sub)?;
        // let mut salt_in_hex = [0u8; 32];
        // hex::decode_to_slice(salt, &mut salt_in_hex)?;

        let call_data = self
            .factory
            .create_account(sub_in_hex.into(), salt, provider)
            .calldata()
            .ok_or(anyhow!("Convert call data to bytes failed!"))?;

        let ret = ethers::abi::encode_packed(&[
            Token::Address(self.factory.address()),
            Token::Bytes(call_data.to_vec()),
        ])
        .map_err(|e| Error::from(e))?;

        Ok(ret.into())
    }

    pub async fn get_address(
        &self,
        sub: String,
        salt: String,
        iss: String,
        aud: String,
    ) -> Result<Address> {
        let provider = get_provider_hashed(iss, aud);
        let sub_in_hex = hex::decode(sub)?;
        let mut salt_in_hex = [0u8; 32];
        hex::decode_to_slice(salt, &mut salt_in_hex)?;

        let address = self
            .factory
            .get_address(sub_in_hex.into(), salt_in_hex, provider)
            .call()
            .await?;
        Ok(address)
    }

    pub async fn create_wallet(
        &self,
        sub: String,
        salt: String,
        iss: String,
        aud: String,
    ) -> Result<Address> {
        let provider = get_provider_hashed(iss, aud);
        let sub_in_hex = hex::decode(sub)?;
        let mut salt_in_hex = [0u8; 32];
        hex::decode_to_slice(salt, &mut salt_in_hex)?;

        let receipt = self
            .factory
            .create_account(sub_in_hex.into(), salt_in_hex, provider)
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow!("Tx Receipt is None"))?;
        for log in receipt.logs {
            if let Ok(event) = NemoAccountCreatedFilter::decode_log(&log.into()) {
                let ret = event.account_implementation;
                return Ok(ret);
            }
        }
        Err(anyhow!("NemoAccountCreated event not in logs"))
    }

    pub fn connect(&self, signer: Arc<M>) -> Self {
        let factory = Factory::from(self.factory.connect(Arc::clone(&signer)));
        let entry_point = EntryPoint::from(self.entry_point.connect(Arc::clone(&signer)));
        Self {
            factory: Arc::new(factory),
            entry_point: Arc::new(entry_point),
            beneficiaries: self.beneficiaries.clone(),
        }
    }

    pub async fn is_created(&self, address: Address) -> bool {
        let factory = Factory::new(address, self.signer());
        factory.address() == self.factory.address()
    }

    pub fn signer(&self) -> Arc<M> {
        Arc::clone(&self.entry_point.client())
    }

    pub fn entry_point(&self) -> Arc<EntryPoint<M>> {
        Arc::clone(&self.entry_point)
    }

    pub fn factory(&self) -> Arc<Factory<M>> {
        Arc::clone(&self.factory)
    }

    pub fn pick_up_beneficiary(&self) -> Address {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.beneficiaries.len());
        self.beneficiaries[index].clone()
    }
}
