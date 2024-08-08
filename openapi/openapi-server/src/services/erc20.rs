use {
    super::utils::init_contract_wallet,
    crate::{
        config::TelegramAuthConfig,
        error::{into_anyhow, Result},
    },
    anyhow::anyhow,
    ethers::{
        types::{
            transaction::eip2718::TypedTransaction, Address, BlockNumber,
            Eip1559TransactionRequest, TransactionRequest,
        },
        utils::parse_ether,
    },
    ethers_contract::ContractFactory,
    ethers_providers::{Http, Middleware, Provider},
    openapi_ethers::erc20::{erc20_bytecode, ERC20 as ERC20Contract, ERC20_ABI},
    openapi_logger::debug,
    openapi_proto::erc20_service::{erc20_server::Erc20, *},
    std::sync::Arc,
    tonic::{Request, Response},
    zion_aa::{
        address_to_string,
        contract_wallet::client::{Client as EthereumClient, ClientMethods},
    },
};

#[derive(Debug, Clone)]
pub struct Erc20Service {
    zion_provider: Arc<Provider<Http>>,
    torii_provider: Arc<Provider<Http>>,
    tele_auth_config: TelegramAuthConfig,
}

impl Erc20Service {
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
impl Erc20 for Erc20Service {
    async fn deploy(&self, request: Request<DeployRequest>) -> Result<Response<DeployResponse>> {
        debug!("{request:?}");
        let header_metadata = request.metadata().clone();
        let DeployRequest {
            name,
            symbol,
            initial_supply,
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
                return Err(into_anyhow(anyhow!(
                    "Transaction fund for deployment failed"
                )));
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

        let initial_supply = parse_ether(initial_supply).map_err(|e| into_anyhow(e.into()))?;
        debug!("initial_supply: {initial_supply:?}");

        let factory = ContractFactory::new(
            ERC20_ABI.clone(),
            erc20_bytecode(),
            Arc::clone(&random_client),
        );

        debug!("Waiting for deploy ERC20 contract...");
        let contract = factory
            .deploy((contract_wallet.address(), name, symbol, initial_supply))
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
            debug!("refund txhash: {}", refund_receipt.transaction_hash);

            let after_refund_balance_of_contract_wallet = self
                .zion_provider
                .get_balance(contract_wallet.address(), Some(BlockNumber::Latest.into()))
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            debug!(
                "Balance of contract wallet after refund: {}",
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
        debug!("{request:?}");

        let TotalSupplyRequest { contract } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.zion_provider));
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
        let rpc_endpoint = self.torii_provider.url().as_str();
        let header_metadata = request.metadata();
        let mut contract_wallet =
            init_contract_wallet(header_metadata, rpc_endpoint, &self.tele_auth_config)
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
        let amount = parse_ether(amount).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .approve(spender_address, amount)
            .calldata()
            .ok_or(into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new()
            .to(contract_address)
            .data(calldata);

        let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
        contract_wallet
            .validate_and_set_pin_code(pin_code.clone(), !has_pin_code, None)
            .await
            .map_err(into_anyhow)?;

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash;
        let txhash_string = format!("{:#x}", txhash);

        debug!("approve txhash: {}", txhash_string);

        Ok(Response::new(ApproveResponse {
            txhash: txhash_string,
        }))
    }

    async fn balance_of(
        &self,
        request: Request<BalanceOfRequest>,
    ) -> Result<Response<BalanceOfResponse>> {
        let BalanceOfRequest { contract, account } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let account_address = account
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let amount = contract
            .balance_of(account_address)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        debug!(
            "contract: {:#x} balance of {:#x}: {}",
            contract_address, account_address, amount
        );

        Ok(Response::new(BalanceOfResponse { amount }))
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

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let remaining_amount = contract
            .allowance(owner_address, spender_address)
            .legacy()
            .call()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        debug!(
            "contract: {:#x} allowance owner: {:#x} spender: {:#x} amount: {}",
            contract_address, owner_address, spender_address, remaining_amount
        );

        Ok(Response::new(AllowanceResponse { remaining_amount }))
    }

    async fn transfer(
        &self,
        request: Request<TransferRequest>,
    ) -> Result<Response<TransferResponse>> {
        let rpc_endpoint = self.torii_provider.url().as_str();
        let header_metadata = request.metadata();
        let mut contract_wallet =
            init_contract_wallet(header_metadata, rpc_endpoint, &self.tele_auth_config)
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
        let amount = parse_ether(amount).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .transfer(recipient_address, amount)
            .calldata()
            .ok_or(into_anyhow(anyhow!("Calldata is None")))?;

        let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
        contract_wallet
            .validate_and_set_pin_code(pin_code.clone(), !has_pin_code, None)
            .await
            .map_err(into_anyhow)?;

        let txhash = contract_wallet
            .send_transaction(
                Eip1559TransactionRequest::new()
                    .to(contract_address)
                    .data(calldata),
                None,
            )
            .await
            .map_err(into_anyhow)?
            .transaction_hash;
        let txhash_string = format!("{:#x}", txhash);

        debug!("transfer txhash: {}", txhash_string);

        Ok(Response::new(TransferResponse {
            txhash: txhash_string,
        }))
    }

    async fn transfer_from(
        &self,
        request: Request<TransferFromRequest>,
    ) -> Result<Response<TransferFromResponse>> {
        let rpc_endpoint = self.torii_provider.url().as_str();
        let header_metadata = request.metadata();
        let mut contract_wallet =
            init_contract_wallet(header_metadata, rpc_endpoint, &self.tele_auth_config)
                .await
                .map_err(into_anyhow)?;

        let TransferFromRequest {
            contract,
            sender,
            recipient,
            amount,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let sender_address = sender
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let recipient_address = recipient
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let amount = parse_ether(amount).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .transfer_from(sender_address, recipient_address, amount)
            .calldata()
            .ok_or(into_anyhow(anyhow!("Calldata is None")))?;

        let has_pin_code = contract_wallet.has_pin_code().await.map_err(into_anyhow)?;
        contract_wallet
            .validate_and_set_pin_code(pin_code.clone(), !has_pin_code, None)
            .await
            .map_err(into_anyhow)?;

        let txhash = contract_wallet
            .send_transaction(
                Eip1559TransactionRequest::new()
                    .to(contract_address)
                    .data(calldata),
                None,
            )
            .await
            .map_err(into_anyhow)?
            .transaction_hash;
        let txhash_string = format!("{:#x}", txhash);
        debug!("transfer_from txhash: {}", txhash_string);

        Ok(Response::new(TransferFromResponse {
            txhash: txhash_string,
        }))
    }
}
