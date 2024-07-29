use std::sync::Arc;

use crate::{
    contracts::{Account, EntryPoint, Factory},
    signer::keys::{key_jwt::KeyJWT, pincode::PINCode, KeyBase},
    types::{jwt::JWTOptions, user_operation::UserOperationSigned},
    utils::{fill_user_op, make_pin_code_holder},
};
use anyhow::{anyhow, Result};
use ethers::{
    signers::Signer,
    types::{Address, Bytes, Eip1559TransactionRequest, TransactionReceipt, H160, U256},
};
use ethers_providers::Middleware;

use super::{operator::Operator, sign::fill_and_sign};

pub struct ContractWallet<M> {
    contract: Arc<Account<M>>,
    pin_code: Option<Arc<PINCode>>,
    jwt_proof: Option<Arc<KeyJWT<M>>>,
    operator: Arc<Operator<M>>,
}

impl<M: Middleware + Signer + 'static> ContractWallet<M> {
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

    pub fn salt(&self) -> Option<[u8; 32]> {
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
        code: Bytes,
        set_onchain: bool,
        // options: Option<ethers::>,
    ) -> Result<()> {
        if self.jwt_proof.is_none() {
            return Err(anyhow!("Uninitialized JWT"));
        }
        if !self.operator.is_created(self.address()).await {
            return Err(anyhow!("Wallet not created"));
        }

        let pin_code_holder = make_pin_code_holder(&code, &self.salt().unwrap())?;
        let mut pin_code_onchain = self.get_pin_code_holder().await?;

        if set_onchain && pin_code_holder.address() != pin_code_onchain {
            let _ = self.onchain_update_pin_code(code).await?;
            pin_code_onchain = self.get_pin_code_holder().await?;
        }

        if pin_code_holder.address() != pin_code_onchain {
            return Err(anyhow!("Invalid PIN Code"));
        }

        self.pin_code = Some(Arc::new(PINCode::new(Arc::new(pin_code_holder))));

        Ok(())
    }

    pub fn set_jwt(&mut self, options: JWTOptions<M>) -> &mut Self {
        self.jwt_proof = Some(Arc::new(KeyJWT::new(options)));
        self
    }

    pub async fn create(
        &self,
        // options: Option<CallOptions>,
        chain_id: Option<U256>,
    ) -> Result<TransactionReceipt> {
        let default_chain_id = self.entry_point().client().get_chainid().await?;
        let chain_id = chain_id.unwrap_or(default_chain_id);

        if self.jwt_proof.is_none() {
            return Err(anyhow!("Uninitialized JWT"));
        }
        if self.is_writeable().await {
            return Err(anyhow!("Wallet already exists"));
        }

        let mut op1 = UserOperationSigned::default();
        let mut_op1 = op1.mut_inner();
        mut_op1.sender = self.contract.address();
        mut_op1.nonce = U256::zero();
        mut_op1.init_code = self.operator.get_init_code(
            self.sub().unwrap(),
            self.salt().unwrap(),
            self.iss().unwrap(),
            self.aud().unwrap(),
        )?;
        mut_op1.signature = vec![0u8; 1].into();

        // if let Some(options) = options {
        //     if let Some(max_fee_per_gas) = options.max_fee_per_gas {
        //         op1.max_fee_per_gas = max_fee_per_gas;
        //     }
        //     if let Some(max_priority_fee_per_gas) = options.max_priority_fee_per_gas {
        //         op1.max_priority_fee_per_gas = max_priority_fee_per_gas;
        //     }
        // }

        let op = fill_user_op(op1.into(), Arc::clone(&self.entry_point()))
            .await?
            .into_inner();

        let signed_tx = self
            .entry_point()
            .handle_ops(vec![op], self.operator.pick_up_beneficiary())
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow!("Tx Receipt is None"))?;

        // if let Some(options) = options {
        //     if let Some(gas_limit) = options.gas_limit {
        //         handle_ops_transaction.gas = Some(gas_limit);
        //     }
        //     if let Some(gas_price) = options.gas_price {
        //         handle_ops_transaction.gas_price = Some(gas_price);
        //     }
        // }

        // let signed_tx = self
        //     .signer()
        //     .send_transaction(handle_ops_transaction)
        //     .await?;
        // signed_tx.await?;

        Ok(signed_tx)
    }

    pub async fn onchain_update_pin_code(
        &self,
        code: Bytes,
        // options: Option<CallOptions>,
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

        let pin_code_holder =
            make_pin_code_holder(&code, &self.jwt_proof.as_ref().unwrap().inner.salt)?;

        let tx_exec_data = self
            .contract
            .update_pin_code(pin_code_holder.address())
            .calldata()
            .ok_or_else(|| anyhow!("Calldata is None!"))?;

        let mut op1 = UserOperationSigned::default();
        let mut_op1 = op1.mut_inner();
        mut_op1.sender = self.contract.address();
        mut_op1.nonce = self.nonce().await?;
        mut_op1.call_data = tx_exec_data;

        // if let Some(options) = options {
        //     if let Some(max_fee_per_gas) = options.max_fee_per_gas {
        //         op1.max_fee_per_gas = max_fee_per_gas;
        //     }
        //     if let Some(max_priority_fee_per_gas) = options.max_priority_fee_per_gas {
        //         op1.max_priority_fee_per_gas = max_priority_fee_per_gas;
        //     }
        // }

        let mut signers: Vec<Arc<dyn KeyBase>> = Vec::new();
        if let Some(pin_code) = &self.pin_code {
            signers.push(Arc::clone(pin_code) as Arc<dyn KeyBase>);
        }
        if let Some(jwt_proof) = &self.jwt_proof {
            signers.push(Arc::clone(jwt_proof) as Arc<dyn KeyBase>);
        }
        if signers.is_empty() {
            return Err(anyhow!("Signers not set yet!"));
        }

        let op = fill_and_sign(op1.into(), signers, self.entry_point(), chain_id)
            .await?
            .into_inner();

        let signed_tx = self
            .entry_point()
            .handle_ops(vec![op], self.operator.pick_up_beneficiary())
            .send()
            .await?
            .await?
            .ok_or_else(|| anyhow!("Tx Receipt is None"))?;

        // if let Some(options) = options {
        //     if let Some(gas_limit) = options.gas_limit {
        //         handle_ops_transaction.gas = Some(gas_limit);
        //     }
        //     if let Some(gas_price) = options.gas_price {
        //         handle_ops_transaction.gas_price = Some(gas_price);
        //     }
        // }

        // let signed_tx = self
        //     .signer()
        //     .send_transaction(handle_ops_transaction)
        //     .await?;
        // signed_tx.await?;

        Ok(signed_tx)
    }

    pub async fn populate_transaction(
        &self,
        transaction: Eip1559TransactionRequest,
        chain_id: Option<U256>,
    ) -> Result<Eip1559TransactionRequest> {
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

        let mut op1 = UserOperationSigned::default();
        let mut_op1 = op1.mut_inner();

        mut_op1.sender = self.contract.address();
        mut_op1.nonce = self.nonce().await?;
        mut_op1.call_data = tx_exec_data;

        if let Some(max_fee_per_gas) = transaction.max_fee_per_gas {
            mut_op1.max_fee_per_gas = max_fee_per_gas;
        }
        if let Some(max_priority_fee_per_gas) = transaction.max_priority_fee_per_gas {
            mut_op1.max_priority_fee_per_gas = max_priority_fee_per_gas;
        }

        let mut signers: Vec<Arc<dyn KeyBase>> = Vec::new();
        if let Some(pin_code) = &self.pin_code {
            signers.push(Arc::clone(pin_code) as Arc<dyn KeyBase>);
        }
        if let Some(jwt_proof) = &self.jwt_proof {
            signers.push(Arc::clone(jwt_proof) as Arc<dyn KeyBase>);
        }
        if signers.is_empty() {
            return Err(anyhow!("Signers not set yet!"));
        }

        let op = fill_and_sign(op1.into(), signers, self.entry_point(), chain_id)
            .await?
            .into_inner();

        let ret = self
            .entry_point()
            .handle_ops([op].into(), self.operator.pick_up_beneficiary());

        if let Some(gas_limit) = transaction.gas {
            ret.gas(gas_limit);
        }
        if let Ok(gas_price) = transaction.get_gas_price().await {
            ret.gas_price(gas_price);
        }

        Ok(ret)
    }

    // pub async fn send_transaction(
    //     &self,
    //     transaction: TransactionRequest,
    //     chain_id: Option<u64>,
    // ) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
    //     let handle_ops_transaction = self.populate_transaction(transaction, chain_id).await?;

    //     let signed_tx = self
    //         .signer()
    //         .send_transaction(handle_ops_transaction)
    //         .await?;
    //     signed_tx.await?;

    //     Ok(signed_tx)
    // }
}
