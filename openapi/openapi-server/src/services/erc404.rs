use {
    super::utils::init_contract_wallet,
    crate::{
        config::TelegramAuthConfig,
        error::{into_anyhow, Result},
    },
    anyhow::anyhow,
    ethers::types::{
        transaction::eip2718::TypedTransaction, Address, BlockNumber, Eip1559TransactionRequest,
        TransactionRequest, U256,
    },
    ethers_contract::ContractFactory,
    ethers_providers::{Http, Middleware, Provider},
    openapi_ethers::erc404::{erc404_bytecode, ERC404 as ERC404Contract, ERC404_ABI},
    openapi_logger::debug,
    openapi_proto::erc404_service::{erc404_server::Erc404, SafeTransferFromRequest, *},
    std::sync::Arc,
    tonic::{Request, Response},
    zion_aa::{
        address_to_string,
        contract_wallet::client::{Client as EthereumClient, ClientMethods},
    },
};

#[derive(Debug, Clone)]
pub struct Erc404Service {
    zion_provider: Arc<Provider<Http>>,
    torii_provider: Arc<Provider<Http>>,
    tele_auth_config: TelegramAuthConfig,
}

impl Erc404Service {
    pub fn new(
        zion_provider: Arc<Provider<Http>>,
        torii_provider: Arc<Provider<Http>>,
        tele_auth_config: TelegramAuthConfig,
    ) -> Self {
        Self {
            zion_provider,
            torii_provider,
            tele_auth_config,
        }
    }
}

#[tonic::async_trait]
impl Erc404 for Erc404Service {
    async fn deploy(&self, request: Request<DeployRequest>) -> Result<Response<DeployResponse>> {
        debug!("{request:?}");
        let header_metadata = request.metadata().clone();
        let DeployRequest {
            name,
            symbol,
            decimals,
            units,
            ids,
            uri,
            pin_code,
        } = request.into_inner();

        let chain_id = self
            .zion_provider
            .get_chainid()
            .await
            .map_err(|e| into_anyhow(e.into()))?;
        let zion_rpc_endpoint = self.zion_provider.url().as_str();
        let torii_rpc_endpoint = self.torii_provider.url().as_str();

        let random_client = Arc::new(
            EthereumClient::random_wallet(zion_rpc_endpoint, chain_id.as_u64())
                .await
                .map_err(into_anyhow)?,
        );

        let mut contract_wallet =
            init_contract_wallet(&header_metadata, torii_rpc_endpoint, &self.tele_auth_config)
                .await
                .map_err(into_anyhow)?;
        debug!("contract wallet address: {:#x}", contract_wallet.address());

        // This session for contract wallet fund to deploy the contract
        {
            let before_balance = self
                .zion_provider
                .get_balance(contract_wallet.address(), Some(BlockNumber::Latest.into()))
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            debug!(
                "Balance of contract wallet before fund for deployment: {}",
                before_balance
            );

            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code.clone(), !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");

            let deployment_fund =
                ethers::utils::parse_ether("0.00001").map_err(|e| into_anyhow(e.into()))?;

            let fund_to_deploy_transaction = Eip1559TransactionRequest::new()
                .to(random_client.address())
                .value(deployment_fund);

            debug!("Waiting for fund for deployment...");
            let receipt = contract_wallet
                .send_transaction(fund_to_deploy_transaction, None)
                .await
                .map_err(into_anyhow)?;

            if receipt.status.unwrap().is_zero() {
                return Err(into_anyhow(anyhow!("Transaction failed")));
            }

            debug!(
                "Contract wallet funded for deployment: {:#x}",
                receipt.transaction_hash
            );
            let after_balance = self
                .zion_provider
                .get_balance(contract_wallet.address(), Some(BlockNumber::Latest.into()))
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            debug!(
                "Balance of contract wallet after fund for deployment: {}",
                after_balance
            );
            let random_wallet_balance = self
                .zion_provider
                .get_balance(random_client.address(), Some(BlockNumber::Latest.into()))
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            debug!(
                "Balance of random wallet after fund for deployment: {}",
                random_wallet_balance
            );
        }

        let units = U256::from_dec_str(&units).map_err(|e| into_anyhow(e.into()))?;
        let ids = ids
            .into_iter()
            .map(|id| U256::from_dec_str(&id).map_err(|e| into_anyhow(e.into())))
            .collect::<Result<Vec<U256>>>()?;

        debug!(
            "owner: {:?}, units: {units:?}, ids: {ids:?}, uri: {uri}",
            contract_wallet.address()
        );

        let factory = ContractFactory::new(
            ERC404_ABI.clone(),
            erc404_bytecode(),
            Arc::clone(&random_client),
        );

        let contract = factory
            .deploy((
                contract_wallet.address(),
                name,
                symbol,
                decimals as u8,
                units,
                ids,
                uri,
            ))
            .map_err(|e| into_anyhow(e.into()))?
            .legacy()
            .send()
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        let contract_address = address_to_string!(contract.address());
        debug!("contract address: {}", contract_address);

        // This session for refund remaining contract deployment amount to contract wallet
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code.clone(), !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;

            let remaining_fund = self
                .zion_provider
                .get_balance(random_client.address(), Some(BlockNumber::Latest.into()))
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            debug!("remaining fund after deployment: {}", remaining_fund);

            let estimate_gas_refund_transaction = TransactionRequest::new()
                .to(contract_wallet.address())
                .from(random_client.address())
                .value(remaining_fund);

            let before_contract_wallet_balance = self
                .zion_provider
                .get_balance(contract_wallet.address(), Some(BlockNumber::Latest.into()))
                .await
                .map_err(|e| into_anyhow(e.into()))?;

            let gas_price = self
                .zion_provider
                .get_gas_price()
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            let gas_limit = random_client
                .estimate_gas(
                    &TypedTransaction::Legacy(estimate_gas_refund_transaction),
                    Some(BlockNumber::Latest.into()),
                )
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            let gas_fee = gas_price * gas_limit;
            debug!("gas_fee: {}", gas_fee);
            debug!("refund amount: {}", remaining_fund - gas_fee);
            debug!(
                "estimated contract wallet balance after refund: {}",
                before_contract_wallet_balance + (remaining_fund - gas_fee)
            );

            if remaining_fund <= gas_fee {
                return Err(into_anyhow(anyhow!(
                    "Insufficient remaining fund to cover the gas fee."
                )));
            }

            let refund_transaction = TransactionRequest::new()
                .to(contract_wallet.address())
                .from(random_client.address())
                .gas(gas_limit)
                .gas_price(gas_price)
                .value(remaining_fund - gas_fee);

            debug!("Waiting for refund...");
            let refund_receipt = random_client
                .send_transaction(refund_transaction, None)
                .await
                .map_err(|e| into_anyhow(e.into()))?
                .await
                .map_err(|e| into_anyhow(e.into()))?
                .ok_or_else(|| into_anyhow(anyhow!("refund receipt is None")))?;

            if refund_receipt.status.unwrap().is_zero() {
                return Err(into_anyhow(anyhow!("refund failed")));
            }
            debug!("refund txhash: {:#x}", refund_receipt.transaction_hash);

            let after_refund_balance_of_contract_wallet = self
                .zion_provider
                .get_balance(contract_wallet.address(), Some(BlockNumber::Latest.into()))
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            debug!(
                "Actual balance of contract wallet after refund: {}",
                after_refund_balance_of_contract_wallet
            );
            let after_refund_balance_of_random_wallet = self
                .zion_provider
                .get_balance(random_client.address(), Some(BlockNumber::Latest.into()))
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            debug!(
                "Balance of random wallet after refund: {}",
                after_refund_balance_of_random_wallet
            );
        }

        Ok(Response::new(DeployResponse {
            contract: contract_address,
        }))
    }

    async fn total_supply(
        &self,
        request: Request<TotalSupplyRequest>,
    ) -> Result<Response<TotalSupplyResponse>> {
        let TotalSupplyRequest { contract } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let total_supply = contract
            .total_supply()
            .legacy()
            .call()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        Ok(Response::new(TotalSupplyResponse { total_supply }))
    }

    async fn approve(&self, request: Request<ApproveRequest>) -> Result<Response<ApproveResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let ApproveRequest {
            contract,
            spender,
            amount,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let spender_address = spender
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let amount = U256::from_dec_str(&amount).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .approve(spender_address, amount)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new()
            .to(contract.address())
            .data(calldata);

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash
            .to_string();

        Ok(Response::new(ApproveResponse { txhash }))
    }

    async fn balance_of(
        &self,
        request: Request<BalanceOfRequest>,
    ) -> Result<Response<BalanceOfResponse>> {
        let BalanceOfRequest {
            contract,
            account,
            token_id,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let account_address = account
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));

        let amount = if let Some(token_id) = token_id {
            let token_id = U256::from_dec_str(&token_id).map_err(|e| into_anyhow(e.into()))?;
            contract
                .balance_of_with_id(account_address, token_id)
                .legacy()
                .await
                .map_err(|e| into_anyhow(e.into()))?
                .to_string()
        } else {
            contract
                .balance_of(account_address)
                .legacy()
                .await
                .map_err(|e| into_anyhow(e.into()))?
                .to_string()
        };

        Ok(Response::new(BalanceOfResponse { amount }))
    }

    async fn balance_of_batch(
        &self,
        request: Request<BalanceOfBatchRequest>,
    ) -> Result<Response<BalanceOfBatchResponse>> {
        let BalanceOfBatchRequest {
            contract,
            accounts,
            token_ids,
        } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let accounts = accounts
            .into_iter()
            .map(|address| {
                address
                    .parse::<Address>()
                    .map_err(|e| into_anyhow(e.into()))
            })
            .collect::<Result<Vec<Address>>>()?;
        let token_ids = token_ids
            .into_iter()
            .map(|id| U256::from_dec_str(&id).map_err(|e| into_anyhow(e.into())))
            .collect::<Result<Vec<U256>>>()?;

        debug!("contract address: {contract_address:?}, accounts: {accounts:?}, token_ids: {token_ids:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let batch_balances = contract
            .balance_of_batch(accounts, token_ids)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?;
        let batch_balances = batch_balances.into_iter().map(|b| b.to_string()).collect();

        Ok(Response::new(BalanceOfBatchResponse { batch_balances }))
    }

    async fn allowance(
        &self,
        request: Request<AllowanceRequest>,
    ) -> Result<Response<AllowanceResponse>> {
        let AllowanceRequest {
            contract,
            owner,
            spender,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let owner_address = owner
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let spender_address = spender
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let remaining_amount = contract
            .allowance(owner_address, spender_address)
            .legacy()
            .call()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        Ok(Response::new(AllowanceResponse { remaining_amount }))
    }

    async fn transfer(
        &self,
        request: Request<TransferRequest>,
    ) -> Result<Response<TransferResponse>> {
        let mut contract_wallet = init_contract_wallet(
            request.metadata(),
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let TransferRequest {
            contract,
            recipient,
            amount,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let recipient_address = recipient
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let amount = U256::from_dec_str(&amount).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .transfer(recipient_address, amount)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Calldata is None")))?;

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let transaction = Eip1559TransactionRequest::new()
            .to(contract.address())
            .data(calldata);

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash;

        let txhash = format!("{:#x}", txhash);
        debug!("safe_transfer_from txhash: {txhash}");

        Ok(Response::new(TransferResponse { txhash }))
    }

    async fn transfer_from(
        &self,
        request: Request<TransferFromRequest>,
    ) -> Result<Response<TransferFromResponse>> {
        let mut contract_wallet = init_contract_wallet(
            request.metadata(),
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let TransferFromRequest {
            contract,
            from,
            to,
            value,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let sender_address = from.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let recipient_address = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let amount = U256::from_dec_str(&value).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .transfer_from(sender_address, recipient_address, amount)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Calldata is None")))?;

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let txhash = contract_wallet
            .send_transaction(
                Eip1559TransactionRequest::new()
                    .to(contract.address())
                    .data(calldata),
                None,
            )
            .await
            .map_err(into_anyhow)?
            .transaction_hash;

        let txhash = format!("{:#x}", txhash);
        debug!("safe_transfer_from txhash: {txhash}");

        Ok(Response::new(TransferFromResponse { txhash }))
    }

    async fn set_approval_for_all(
        &self,
        request: Request<SetApprovalForAllRequest>,
    ) -> Result<Response<SetApprovalForAllResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let SetApprovalForAllRequest {
            contract,
            operator,
            approved,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let operator_address = operator
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, operator: {operator_address:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .set_approval_for_all(operator_address, approved)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new()
            .to(contract.address())
            .data(calldata);

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash;

        let txhash = format!("{:#x}", txhash);
        debug!("safe_transfer_from txhash: {txhash}");

        Ok(Response::new(SetApprovalForAllResponse { txhash }))
    }

    async fn is_approved_for_all(
        &self,
        request: Request<IsApprovedForAllRequest>,
    ) -> Result<Response<IsApprovedForAllResponse>> {
        let IsApprovedForAllRequest {
            contract,
            owner,
            operator,
        } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let owner_adress = owner
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let operator_address = operator
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let ret = contract
            .is_approved_for_all(owner_adress, operator_address)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        Ok(Response::new(IsApprovedForAllResponse { result: ret }))
    }

    async fn safe_transfer_from(
        &self,
        request: Request<SafeTransferFromRequest>,
    ) -> Result<Response<SafeTransferFromResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let SafeTransferFromRequest {
            contract,
            from,
            to,
            token_id,
            value,
            data,
            pin_code,
        } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let from = from.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let to = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_dec_str(&token_id).map_err(|e| into_anyhow(e.into()))?;
        let value = U256::from_dec_str(&value).map_err(|e| into_anyhow(e.into()))?;
        let data = hex::decode(data).map_err(|e| into_anyhow(e.into()))?.into();

        debug!("contract: {contract_address:?}, from: {from:?}, to: {to:?}, token_id: {token_id:?}, value: {value:?}, data: {data:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .safe_transfer_from(from, to, token_id, value, data)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new()
            .to(contract.address())
            .data(calldata);

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash;

        let txhash = format!("{:#x}", txhash);
        debug!("safe_transfer_from txhash: {txhash}");

        Ok(Response::new(SafeTransferFromResponse { txhash }))
    }

    async fn safe_batch_transfer_from(
        &self,
        request: Request<SafeBatchTransferFromRequest>,
    ) -> Result<Response<SafeBatchTransferFromResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let SafeBatchTransferFromRequest {
            contract,
            from,
            to,
            token_ids,
            values,
            data,
            pin_code,
        } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let from = from.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let to = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let token_ids = token_ids
            .iter()
            .map(|t| U256::from_dec_str(t).map_err(|e| into_anyhow(e.into())))
            .collect::<Result<Vec<_>>>()?;
        let values = values
            .iter()
            .map(|v| U256::from_dec_str(v).map_err(|e| into_anyhow(e.into())))
            .collect::<Result<Vec<_>>>()?;
        let data = hex::decode(data).map_err(|e| into_anyhow(e.into()))?.into();

        debug!("contract: {contract_address:?}, from: {from:?}, to: {to:?}, token_ids: {token_ids:?}, values: {values:?}, data: {data:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .safe_batch_transfer_from(from, to, token_ids, values, data)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new()
            .to(contract.address())
            .data(calldata);

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash;

        let txhash = format!("{:#x}", txhash);
        debug!("safe_transfer_from txhash: {txhash}");

        Ok(Response::new(SafeBatchTransferFromResponse { txhash }))
    }

    async fn erc1155_balance_of(
        &self,
        request: Request<Erc1155BalanceOfRequest>,
    ) -> Result<Response<Erc1155BalanceOfResponse>> {
        let Erc1155BalanceOfRequest {
            contract,
            account,
            token_id,
        } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let account = account
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let toke_id = U256::from_dec_str(&token_id).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, account: {account:?}, token_id: {toke_id:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let amount = contract
            .erc_1155_balance_of(account, toke_id)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        Ok(Response::new(Erc1155BalanceOfResponse { amount }))
    }

    async fn erc20_balance_of(
        &self,
        request: Request<Erc20BalanceOfRequest>,
    ) -> Result<Response<Erc20BalanceOfResponse>> {
        let Erc20BalanceOfRequest { contract, account } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let account_address = account
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let amount = contract
            .erc_20_balance_of(account_address)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        Ok(Response::new(Erc20BalanceOfResponse { amount }))
    }

    async fn erc1155_transfer_exempt(
        &self,
        request: Request<Erc1155TransferExemptRequest>,
    ) -> Result<Response<Erc1155TransferExemptResponse>> {
        let Erc1155TransferExemptRequest { contract, target } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let target = target
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let is_exempt = contract
            .erc_1155_transfer_exempt(target)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        Ok(Response::new(Erc1155TransferExemptResponse { is_exempt }))
    }

    async fn add_transfer_exempt(
        &self,
        request: Request<AddTransferExemptRequest>,
    ) -> Result<Response<AddTransferExemptResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let AddTransferExemptRequest {
            contract,
            target,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let target = target
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {:#x}, target: {:#x}", contract_address, target);

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .add_transfer_exempt(target)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new()
            .to(contract.address())
            .data(calldata);

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash;

        let txhash = format!("{:#x}", txhash);
        debug!("safe_transfer_from txhash: {txhash}");

        Ok(Response::new(AddTransferExemptResponse { txhash }))
    }

    async fn remove_transfer_exempt(
        &self,
        request: Request<RemoveTransferExemptRequest>,
    ) -> Result<Response<RemoveTransferExemptResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let RemoveTransferExemptRequest {
            contract,
            target,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let target = target
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {:#x}, target: {:#x}", contract_address, target);

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .remove_transfer_exempt(target)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new()
            .to(contract.address())
            .data(calldata);

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash;

        let txhash = format!("{:#x}", txhash);
        debug!("safe_transfer_from txhash: {txhash}");

        Ok(Response::new(RemoveTransferExemptResponse { txhash }))
    }

    async fn mint(&self, request: Request<MintRequest>) -> Result<Response<MintResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let MintRequest {
            contract,
            account,
            amount,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let account = account
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let amount = U256::from_dec_str(&amount).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, account: {account:?}, amount: {amount:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .mint(account, amount)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Mint calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new()
            .to(contract.address())
            .data(calldata);

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash;

        let txhash = format!("{:#x}", txhash);
        debug!("mint txhash: {txhash}");

        Ok(Response::new(MintResponse { txhash }))
    }

    async fn burn(&self, request: Request<BurnRequest>) -> Result<Response<BurnResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let BurnRequest {
            contract,
            amount,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let amount = U256::from_dec_str(&amount).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, amount: {amount:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .burn(amount)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Burn calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new()
            .to(contract.address())
            .data(calldata);

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash;

        let txhash = format!("{:#x}", txhash);
        debug!("Burn txhash: {txhash}");

        Ok(Response::new(BurnResponse { txhash }))
    }

    async fn burn_from(
        &self,
        request: Request<BurnFromRequest>,
    ) -> Result<Response<BurnFromResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let BurnFromRequest {
            contract,
            account,
            amount,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let account = account
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let amount = U256::from_dec_str(&amount).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, account: {account:?}, amount: {amount:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .burn_from(account, amount)
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("BurnFrom calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new()
            .to(contract.address())
            .data(calldata);

        // This session is used to validate the pin code
        {
            let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
            contract_wallet
                .validate_and_set_pin_code(pin_code, !has_pin_code, None)
                .await
                .map_err(into_anyhow)?;
            debug!("Validated pin code");
        }

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash;

        let txhash = format!("{:#x}", txhash);
        debug!("Burn txhash: {txhash}");

        Ok(Response::new(BurnFromResponse { txhash }))
    }
}
