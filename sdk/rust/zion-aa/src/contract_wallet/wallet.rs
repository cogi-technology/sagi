use std::{collections::HashMap, sync::Arc};

use crate::{
    contracts::{Account, EntryPoint, Factory},
    signer::keys::{key_jwt::KeyJWT, pincode::PINCode, KeyBase},
    types::{
        jwt::JWTOptions,
        user_operation::{request::UserOperationRequest, UserOperationSigned},
    },
    utils::{fill_user_op, make_pin_code_holder},
};
use anyhow::{anyhow, Result};
use ethers::{
    signers::Signer,
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockNumber, Bytes,
        Eip1559TransactionRequest, TransactionReceipt, H160, U256,
    },
};
use ethers_providers::Middleware;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{operator::Operator, sign::fill_and_sign};

pub struct ContractWallet<M, S> {
    contract: Arc<Account<M>>,
    pin_code: Option<Arc<PINCode>>,
    jwt_proof: Option<Arc<KeyJWT<S>>>,
    operator: Arc<Operator<M>>,
}

impl<M, S> ContractWallet<M, S>
where
    M: Middleware + 'static,
    S: Signer + 'static,
{
    pub fn new(contract_wallet_address: Address, operator: Arc<Operator<M>>) -> Self {
        let contract_wallet = Arc::new(Account::new(contract_wallet_address, operator.signer()));
        Self {
            contract: contract_wallet,
            pin_code: None,
            jwt_proof: None,
            operator: Arc::clone(&operator),
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

    pub fn sub(&self) -> Option<String> {
        self.jwt_proof
            .as_ref()
            .map(|jwt| jwt.inner.payload.sub.clone())
    }

    pub fn salt(&self) -> Option<String> {
        self.jwt_proof.as_ref().map(|jwt| jwt.inner.salt.clone())
    }

    pub fn iss(&self) -> Option<String> {
        self.jwt_proof
            .as_ref()
            .map(|jwt| jwt.inner.payload.iss.clone())
    }

    pub fn aud(&self) -> Option<String> {
        self.jwt_proof
            .as_ref()
            .map(|jwt| jwt.inner.payload.aud.clone())
    }

    pub fn get_required_prefund(&self) -> Result<U256> {
        let default_user_op = UserOperationSigned::default().into_inner();

        let required_gas = U256::from(default_user_op.verification_gas_limit)
            + U256::from(default_user_op.pre_verification_gas);
        let ret = required_gas * U256::from(default_user_op.max_fee_per_gas);
        Ok(ret)
    }

    pub async fn is_readonly(&self) -> bool {
        !self.is_writeable().await
    }

    pub async fn is_writeable(&self) -> bool {
        self.operator.is_created(self.contract.address()).await
    }

    pub async fn nonce(&self) -> Result<U256> {
        self.contract.nonce().await.map_err(|e| e.into())
    }

    pub async fn get_pin_code_holder(&self) -> Result<H160> {
        self.contract.pin_code().await.map_err(|e| e.into())
    }

    pub async fn has_pin_code(&self) -> Result<bool> {
        Ok(!self.get_pin_code_holder().await?.is_zero())
    }

    pub async fn validate_and_set_pin_code(
        &mut self,
        code: String,
        set_onchain: bool,
        options: Option<Overrides>,
    ) -> Result<()> {
        if self.jwt_proof.is_none() {
            return Err(anyhow!("Uninitialized JWT"));
        }
        if !self.operator.is_created(self.address()).await {
            return Err(anyhow!("Wallet not created"));
        }
        let salt = self.salt().ok_or(anyhow!("salt is None"))?;

        let pin_code_holder = make_pin_code_holder(code.clone(), salt)?;

        let mut pin_code_onchain = self.get_pin_code_holder().await?;

        if set_onchain && pin_code_holder.address() != pin_code_onchain {
            let _ = self.onchain_update_pin_code(code, options).await?;
            pin_code_onchain = self.get_pin_code_holder().await?;
        }

        if pin_code_holder.address() != pin_code_onchain {
            return Err(anyhow!("Invalid PIN Code"));
        }

        self.pin_code = Some(Arc::new(PINCode::new(Arc::new(pin_code_holder))));

        Ok(())
    }

    pub fn set_jwt(&mut self, options: JWTOptions<S>) -> &mut Self {
        self.jwt_proof = Some(Arc::new(KeyJWT::new(options)));
        self
    }

    pub async fn create(
        &self,
        chain_id: Option<U256>,
        options: Option<Overrides>,
    ) -> Result<TransactionReceipt> {
        let default_chain_id = self.entry_point().client().get_chainid().await?;
        let _chain_id = chain_id.unwrap_or(default_chain_id);

        if self.jwt_proof.is_none() {
            return Err(anyhow!("Uninitialized JWT"));
        }
        if self.is_writeable().await {
            return Err(anyhow!("Wallet already exists"));
        }

        let init_code = self.operator.get_init_code(
            self.sub().ok_or(anyhow!("sub is None"))?,
            self.salt().ok_or(anyhow!("salt is None"))?,
            self.iss().ok_or(anyhow!("iss is None"))?,
            self.aud().ok_or(anyhow!("aud is None"))?,
        )?;
        let signature = vec![rand::thread_rng().r#gen::<u8>(); 1];
        let mut request_op = UserOperationRequest {
            sender: self.contract.address(),
            nonce: U256::zero(),
            init_code: Some(init_code),
            signature: Some(signature.into()),
            ..Default::default()
        };

        if let Some(options) = options.clone() {
            if options.max_fee_per_gas.is_some() {
                request_op.max_fee_per_gas = options.max_fee_per_gas;
            }
            if options.max_priority_fee_per_gas.is_some() {
                request_op.max_priority_fee_per_gas = options.max_priority_fee_per_gas;
            }
        }

        let signed_op = fill_user_op(
            request_op,
            Arc::clone(&self.entry_point().client()),
            self.entry_point().address(),
        )
        .await?
        .into_inner();

        let mut handle_ops_transaction = self
            .entry_point()
            .handle_ops(vec![signed_op], self.operator.pick_up_beneficiary());

        if let Some(options) = options {
            if let Some(gas_limit) = options.gas_limit {
                handle_ops_transaction.tx.set_gas(gas_limit);
            }
            if let Some(gas_price) = options.gas_price {
                handle_ops_transaction.tx.set_gas_price(gas_price);
            }
        }

        let receipt = handle_ops_transaction
            .legacy()
            .send()
            .await?
            .await?
            .unwrap_or_default();

        Ok(receipt)
    }

    pub async fn onchain_update_pin_code(
        &self,
        code: String,
        options: Option<Overrides>,
    ) -> Result<TransactionReceipt> {
        let chain_id = self.entry_point().client().get_chainid().await.unwrap();

        if self.jwt_proof.is_none() {
            return Err(anyhow!("Uninitialized JWT"));
        }
        if !self.operator.is_created(self.address()).await {
            return Err(anyhow!("Wallet not created"));
        }
        if self.has_pin_code().await? && self.pin_code.is_none() {
            return Err(anyhow!("Old PIN Code not setup"));
        }
        let salt = self
            .jwt_proof
            .as_ref()
            .ok_or(anyhow!("jwt_proof signer is None"))?
            .inner
            .salt
            .clone();

        let pin_code_holder = make_pin_code_holder(code, salt)?;

        let tx_exec_data = self
            .contract
            .update_pin_code(pin_code_holder.address())
            .calldata()
            .ok_or_else(|| anyhow!("Calldata is None!"))?;

        let mut request_op = UserOperationRequest {
            sender: self.contract.address(),
            nonce: self.nonce().await?,
            call_data: tx_exec_data,
            ..Default::default()
        };

        if let Some(options) = options.clone() {
            if options.max_fee_per_gas.is_some() {
                request_op.max_fee_per_gas = options.max_fee_per_gas;
            }
            if options.max_priority_fee_per_gas.is_some() {
                request_op.max_priority_fee_per_gas = options.max_priority_fee_per_gas;
            }
        }

        let mut signers: Vec<Arc<dyn KeyBase + Send + Sync>> = Vec::new();
        if let Some(pin_code) = &self.pin_code {
            signers.push(pin_code.clone());
        }
        if let Some(jwt_proof) = &self.jwt_proof {
            signers.push(jwt_proof.clone());
        }
        if signers.is_empty() {
            return Err(anyhow!("Signers not set yet!"));
        }

        let signed_op = fill_and_sign(
            request_op,
            signers,
            self.entry_point().address(),
            Arc::clone(&self.entry_point().client()),
            chain_id,
        )
        .await?
        .into_inner();

        let mut handle_ops_transaction = self
            .entry_point()
            .handle_ops(vec![signed_op], self.operator.pick_up_beneficiary());

        if let Some(options) = options {
            if let Some(gas_limit) = options.gas_limit {
                handle_ops_transaction.tx.set_gas(gas_limit);
            }
            if let Some(gas_price) = options.gas_price {
                handle_ops_transaction.tx.set_gas_price(gas_price);
            }
        }

        let signed_tx = handle_ops_transaction
            .legacy()
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow!("Tx Receipt is None"))?;

        Ok(signed_tx)
    }

    pub async fn populate_transaction(
        &self,
        transaction: Eip1559TransactionRequest,
        chain_id: Option<U256>,
    ) -> Result<TypedTransaction> {
        if transaction.to.is_none() {
            return Err(anyhow!("Transaction to is undefined"));
        }

        let default_chain_id = self.entry_point().client().get_chainid().await.unwrap();
        let chain_id = chain_id.unwrap_or(default_chain_id);
        let tx_value = transaction.value.unwrap_or(U256::zero());
        let tx_data = transaction.data.unwrap_or(Bytes::new());
        let to = transaction
            .to
            .ok_or_else(|| anyhow!("to is None!"))?
            .as_address()
            .ok_or_else(|| anyhow!("To Address is None!"))?
            .to_owned();

        let tx_exec_data = self
            .contract
            .execute(to, tx_value, tx_data)
            .calldata()
            .ok_or_else(|| anyhow!("Calldata is None!"))?;
        // Debug
        // println!(
        //     "ContractWallet::populate_tx::line331::tx_exec_data: {}",
        //     hex::encode(tx_exec_data.clone())
        // );

        let mut request_op = UserOperationRequest {
            nonce: self.nonce().await?,
            sender: self.contract.address(),
            call_data: tx_exec_data,
            ..Default::default()
        };

        if transaction.max_fee_per_gas.is_some() {
            request_op.max_fee_per_gas = transaction.max_fee_per_gas;
        }
        if transaction.max_priority_fee_per_gas.is_some() {
            request_op.max_priority_fee_per_gas = transaction.max_priority_fee_per_gas;
        }

        let mut signers: Vec<Arc<dyn KeyBase + Send + Sync>> = Vec::new();
        if let Some(pin_code) = &self.pin_code {
            signers.push(Arc::clone(pin_code) as Arc<dyn KeyBase + Send + Sync>);
        }
        if let Some(jwt_proof) = &self.jwt_proof {
            signers.push(Arc::clone(jwt_proof) as Arc<dyn KeyBase + Send + Sync>);
        }
        if signers.is_empty() {
            return Err(anyhow!("Signers not set yet!"));
        }

        let signed_op = fill_and_sign(
            request_op,
            signers,
            self.entry_point().address(),
            Arc::clone(&self.entry_point().client()),
            chain_id,
        )
        .await?
        .into_inner();

        // Debug
        // println!("ContractWallet::populate_tx::line373::op: {signed_op:#?}",);

        let ret = if let Some(gas_limit) = transaction.gas {
            self.entry_point()
                .handle_ops([signed_op].into(), self.operator.pick_up_beneficiary())
                .gas(gas_limit)
                .legacy()
                .tx
        } else {
            self.entry_point()
                .handle_ops([signed_op].into(), self.operator.pick_up_beneficiary())
                .legacy()
                .tx
        };

        // Debug
        // println!("ContractWallet::populate_tx::line387::ret: {ret:#?}",);

        Ok(ret)
    }

    pub async fn send_transaction(
        &self,
        transaction: Eip1559TransactionRequest,
        chain_id: Option<U256>,
    ) -> Result<TransactionReceipt> {
        let handle_ops_transaction = self.populate_transaction(transaction, chain_id).await?;

        self.signer()
            .send_transaction(handle_ops_transaction, None)
            .await?
            .await?
            .ok_or(anyhow!("Tx Receipt is None!"))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Overrides {
    pub gas_limit: Option<U256>,
    pub gas_price: Option<U256>,
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
    pub nonce: Option<U256>,
    pub type_: Option<u64>, // `type` is a reserved keyword in Rust
    pub access_list: Option<Vec<AccessListItem>>, // Assuming AccessListish is a Vec of AccessListItem
    pub custom_data: Option<HashMap<String, serde_json::Value>>,
    pub ccip_read_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessListItem {
    pub address: String,
    pub storage_keys: Vec<U256>,
}
