pub mod sign;

use std::sync::Arc;

use ethers::{middleware::SignerMiddleware, signers::LocalWallet, types::Address};
use ethers_providers::{Http, Middleware, Provider};

use crate::{
    contracts::{EntryPoint, Factory},
    types::contract_wallet::ContractWalletOperator,
};

pub type Client = SignerMiddleware<Arc<Provider<Http>>, LocalWallet>;

#[derive(Clone)]
pub struct Operator<M: Middleware> {
    factory: Arc<Factory<M>>,
    entry_point: Arc<EntryPoint<M>>,
    beneficiaries: Vec<Address>,
}

impl<M: Middleware> Operator<M> {
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

    pub fn get_ephemeral_key_pair<M>(endpoint: &str, private_key: Option<&str>) -> Client {
        if endpoint.is_empty() {
            panic!("Endpoint is required");
        }
        if let Some(pk) = private_key {
            M::new_from_private_key(pk, Provider::try_from(endpoint).unwrap())
        } else {
            Wallet::new_random(Provider::try_from(endpoint).unwrap())
        }
    }

    pub fn get_init_code(&self, sub: &str, salt: &str, iss: &str, aud: &str) -> Bytes {
        let sub_in_hex = hex::encode(to_utf8_bytes(sub));
        let provider = get_provider_hashed(iss, aud);
        let call_data = self
            .factory
            .encode("createAccount", (sub_in_hex, salt, provider))
            .unwrap();
        let ret = ethers::abi::encode_packed(&[self.factory.address().into(), call_data.into()]);
        ret
    }

    pub async fn get_address(
        &self,
        sub: &str,
        salt: &str,
        iss: &str,
        aud: &str,
        options: Option<CallOverrides>,
    ) -> Result<Address, Box<dyn std::error::Error>> {
        let sub_in_hex = hex::encode(to_utf8_bytes(sub));
        let provider = get_provider_hashed(iss, aud);
        let args = match options {
            Some(opts) => (sub_in_hex, salt, provider, opts),
            None => (sub_in_hex, salt, provider),
        };
        let address: Address = self.factory.method("getAddress", args)?.call().await?;
        Ok(address)
    }

    pub async fn create_wallet(
        &self,
        sub: &str,
        salt: &str,
        iss: &str,
        aud: &str,
        options: Option<PayableOverrides>,
    ) -> Result<Address, Box<dyn std::error::Error>> {
        let sub_in_hex = hex::encode(to_utf8_bytes(sub));
        let provider = get_provider_hashed(iss, aud);
        let args = match options {
            Some(opts) => (sub_in_hex, salt, provider, opts),
            None => (sub_in_hex, salt, provider),
        };
        let receipt = self
            .factory
            .method("createAccount", args)?
            .send()
            .await?
            .await?;
        let event = receipt
            .logs
            .iter()
            .find(|log| log.event == "NemoAccountCreated")
            .unwrap();
        let address = event.args[0].parse().unwrap();
        Ok(address)
    }

    pub fn connect(&self, signer: Arc<M>) -> Self {
        let factory = self.factory.connect(signer.clone());
        let entry_point = self.entry_point.connect(signer);
        Self {
            factory: Arc::new(factory),
            entry_point: Arc::new(entry_point),
            beneficiaries: self.beneficiaries.clone(),
        }
    }

    pub async fn is_created(&self, address: Address, options: Option<CallOverrides>) -> bool {
        let contract = Contract::new(address, AccountAbi, self.factory.client().clone());
        let factory_address: Address = contract
            .method("factory", options)
            .unwrap()
            .call()
            .await
            .unwrap();
        factory_address == self.factory.address()
    }

    pub fn signer(&self) -> Arc<M> {
        self.entry_point.client()
    }

    pub fn entry_point(&self) -> Arc<Contract<M>> {
        self.entry_point.clone()
    }

    pub fn factory(&self) -> Arc<Contract<M>> {
        self.factory.clone()
    }

    pub fn pick_up_beneficiary(&self) -> &str {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.beneficiaries.len());
        &self.beneficiaries[index]
    }
}
