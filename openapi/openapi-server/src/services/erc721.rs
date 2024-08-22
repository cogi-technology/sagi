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
    openapi_ethers::erc721::{erc721_bytecode, ERC721 as ERC721Contract, ERC721_ABI},
    openapi_logger::debug,
    openapi_proto::erc721_service::{erc721_server::Erc721, *},
    std::sync::Arc,
    tonic::{Request, Response},
    zion_aa::{
        address_to_string,
        contract_wallet::client::{Client as EthereumClient, ClientMethods},
    },
};

#[derive(Debug, Clone)]
pub struct Erc721Service {
    zion_provider: Arc<Provider<Http>>,
    torii_provider: Arc<Provider<Http>>,
    tele_auth_config: TelegramAuthConfig,
}

impl Erc721Service {
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
impl Erc721 for Erc721Service {
    async fn deploy(&self, request: Request<DeployRequest>) -> Result<Response<DeployResponse>> {
        debug!("{request:?}");
        let header_metadata = request.metadata().clone();
        let DeployRequest {
            name,
            symbol,
            base_uri,
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

        let factory = ContractFactory::new(
            ERC721_ABI.clone(),
            erc721_bytecode(),
            Arc::clone(&random_client),
        );

        debug!("Waiting for deploy ERC721 contract...");
        let contract = factory
            .deploy((contract_wallet.address(), name, symbol, base_uri))
            .map_err(|e| into_anyhow(e.into()))?
            .legacy()
            .send()
            .await
            .map_err(|e| into_anyhow(e.into()))?;
        let contract_address = address_to_string!(contract.address());
        debug!("contract address: {}", contract_address);

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
                .ok_or_else(|| into_anyhow(anyhow!("refund receipt is none")))?;

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

        debug!(
            "contract address: {:?}, account address: {:?}",
            contract_address, account_address
        );

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let amount = contract
            .balance_of(account_address)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        Ok(Response::new(BalanceOfResponse { amount }))
    }

    async fn owner_of(
        &self,
        request: Request<OwnerOfRequest>,
    ) -> Result<Response<OwnerOfResponse>> {
        let OwnerOfRequest { contract, token_id } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let owner = contract
            .owner_of(U256::from_dec_str(token_id.as_str()).map_err(|e| into_anyhow(e.into()))?)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        let owner_address = address_to_string!(owner);

        Ok(Response::new(OwnerOfResponse {
            owner: owner_address,
        }))
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
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let from_address = from.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let to_address = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_dec_str(&token_id).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, from: {from_address:?}, to: {to_address:?}, token_id: {token_id:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .safe_transfer_from(from_address, to_address, token_id)
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

    async fn transfer_from(
        &self,
        request: Request<TransferFromRequest>,
    ) -> Result<Response<TransferFromResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let TransferFromRequest {
            contract,
            from,
            to,
            token_id,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let from_address = from.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let to_address = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_dec_str(&token_id).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, from: {from_address:?}, to: {to_address:?}, token_id: {token_id:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .transfer_from(from_address, to_address, token_id)
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
        debug!("transfer_from txhash: {txhash}");

        Ok(Response::new(TransferFromResponse { txhash }))
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
            to,
            token_id,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let to_address = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_dec_str(&token_id).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, to: {to_address:?}, token_id: {token_id:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .approve(to_address, token_id)
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
        debug!("approve txhash: {txhash}");

        Ok(Response::new(ApproveResponse { txhash }))
    }

    async fn award_item(
        &self,
        request: Request<AwardItemRequest>,
    ) -> Result<Response<AwardItemResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let AwardItemRequest {
            contract,
            account,
            cid,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let account_address = account
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, account: {account_address:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .award_item(account_address, cid)
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
        debug!("award_item txhash: {txhash}");

        Ok(Response::new(AwardItemResponse { txhash }))
    }

    async fn award_items(
        &self,
        request: Request<AwardItemsRequest>,
    ) -> Result<Response<AwardItemsResponse>> {
        let header_metadata = request.metadata();
        let mut contract_wallet = init_contract_wallet(
            header_metadata,
            self.torii_provider.url().as_str(),
            &self.tele_auth_config,
        )
        .await
        .map_err(into_anyhow)?;

        let AwardItemsRequest {
            contract,
            accounts,
            cids,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let accounts = accounts
            .into_iter()
            .map(|a| a.parse::<Address>().map_err(|e| into_anyhow(e.into())))
            .collect::<Result<Vec<Address>>>()?;

        debug!("contract: {contract_address:?}, account: {accounts:?}, cids: {cids:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .award_items(accounts, cids)
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
        debug!("award_items txhash: {txhash}");

        Ok(Response::new(AwardItemsResponse { txhash }))
    }

    async fn get_approved(
        &self,
        request: Request<GetApprovedRequest>,
    ) -> Result<Response<GetApprovedResponse>> {
        let GetApprovedRequest { contract, token_id } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_dec_str(&token_id).map_err(|e| into_anyhow(e.into()))?;
        debug!("contract: {contract_address:?}, token_id: {token_id:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let operator = contract
            .get_approved(token_id)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        let operator = address_to_string!(operator);
        debug!("operator: {operator}");

        Ok(Response::new(GetApprovedResponse { operator }))
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

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
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
        debug!("set_approval_for_all txhash: {txhash}");

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

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let ret = contract
            .is_approved_for_all(owner_adress, operator_address)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        Ok(Response::new(IsApprovedForAllResponse { result: ret }))
    }

    async fn token_uri(
        &self,
        request: Request<TokenUriRequest>,
    ) -> Result<Response<TokenUriResponse>> {
        let TokenUriRequest { contract, token_id } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_dec_str(&token_id).map_err(|e| into_anyhow(e.into()))?;
        debug!("contract: {contract_address:?}, token_id: {token_id:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let token_uri = contract
            .token_uri(token_id)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?;
        debug!("token_uri: {token_uri}");

        Ok(Response::new(TokenUriResponse { token_uri }))
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
            token_ids,
            pin_code,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let token_ids = token_ids
            .iter()
            .map(|id| U256::from_dec_str(id).map_err(|e| into_anyhow(e.into())))
            .collect::<Result<Vec<U256>>>()?;

        debug!("contract: {contract_address:?}, token_ids: {token_ids:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.zion_provider));
        let calldata = contract
            .force_burns(token_ids)
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
        debug!("burn txhash: {txhash}");

        Ok(Response::new(BurnResponse { txhash }))
    }
}
