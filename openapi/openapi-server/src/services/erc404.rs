use {
    super::utils::{init_contract_wallet, Result},
    crate::helpers::utils::into_anyhow,
    anyhow::anyhow,
    ethers::{
        types::{Address, Eip1559TransactionRequest, U256},
        utils::parse_ether,
    },
    ethers_contract::ContractFactory,
    openapi_ethers::{
        client::Client as EthereumClient,
        erc404::{erc404_bytecode, ERC404 as ERC404Contract, ERC404_ABI},
    },
    openapi_logger::debug,
    openapi_proto::erc404_service::{erc404_server::Erc404, SafeTransferFromRequest, *},
    serde_json::value,
    std::{str::FromStr, sync::Arc},
    tonic::{Request, Response},
    zion_aa::address_to_string,
};

pub struct Erc404Service {
    client: Arc<EthereumClient>,
}

impl Erc404Service {
    pub fn new(client: Arc<EthereumClient>) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl Erc404 for Erc404Service {
    async fn deploy(&self, request: Request<DeployRequest>) -> Result<Response<DeployResponse>> {
        debug!("{request:?}");

        let DeployRequest {
            owner,
            name,
            symbol,
            initial_supply,
            units,
            ids,
            uri,
        } = request.into_inner();

        let owner = owner
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let initial_supply = parse_ether(initial_supply).map_err(|e| into_anyhow(e.into()))?;
        let units = U256::from_str(&units).map_err(|e| into_anyhow(e.into()))?;
        let ids = ids
            .into_iter()
            .map(|id| U256::from_str(&id).map_err(|e| into_anyhow(e.into())))
            .collect::<Result<Vec<U256>>>()?;

        debug!("owner: {owner:?}, initial_supply: {initial_supply:?}, units: {units:?}, ids: {ids:?}, uri: {uri}");

        let factory = ContractFactory::new(
            ERC404_ABI.clone(),
            erc404_bytecode(),
            Arc::clone(&self.client),
        );

        let contract = factory
            .deploy((name, symbol, initial_supply, units, ids, uri))
            .map_err(|e| into_anyhow(e.into()))?
            .legacy()
            .send()
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        let contract_address = address_to_string!(contract.address());
        debug!("contract address: {}", contract_address);

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

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
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
        let contract_wallet = init_contract_wallet(header_metadata)
            .await
            .map_err(into_anyhow)?;

        let ApproveRequest {
            contract,
            spender,
            amount,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let spender_address = spender
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let amount = parse_ether(amount).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
        let calldata = contract
            .approve(spender_address, amount)
            .calldata()
            .ok_or(into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new().data(calldata);

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
        let BalanceOfRequest { contract, account } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let account_address = account
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
        let amount = contract
            .balance_of(account_address)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

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
            .map(|id| U256::from_str(&id).map_err(|e| into_anyhow(e.into())))
            .collect::<Result<Vec<U256>>>()?;

        debug!("contract address: {contract_address:?}, accounts: {accounts:?}, token_ids: {token_ids:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
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

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
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
        let contract_wallet = init_contract_wallet(request.metadata())
            .await
            .map_err(into_anyhow)?;

        let TransferRequest {
            contract,
            recipient,
            amount,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let recipient_address = recipient
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let amount = parse_ether(amount).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
        let calldata = contract
            .transfer(recipient_address, amount)
            .calldata()
            .ok_or(into_anyhow(anyhow!("Calldata is None")))?;

        let txhash = contract_wallet
            .send_transaction(Eip1559TransactionRequest::new().data(calldata), None)
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .transaction_hash
            .to_string();

        Ok(Response::new(TransferResponse { txhash }))
    }

    async fn transfer_from(
        &self,
        request: Request<TransferFromRequest>,
    ) -> Result<Response<TransferFromResponse>> {
        let contract_wallet = init_contract_wallet(request.metadata())
            .await
            .map_err(into_anyhow)?;

        let TransferFromRequest {
            contract,
            from,
            to,
            value,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let sender_address = from.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let recipient_address = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let amount = parse_ether(value).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
        let calldata = contract
            .transfer_from(sender_address, recipient_address, amount)
            .calldata()
            .ok_or(into_anyhow(anyhow!("Calldata is None")))?;

        let txhash = contract_wallet
            .send_transaction(Eip1559TransactionRequest::new().data(calldata), None)
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .transaction_hash
            .to_string();

        Ok(Response::new(TransferFromResponse { txhash }))
    }

    async fn set_approval_for_all(
        &self,
        request: Request<SetApprovalForAllRequest>,
    ) -> Result<Response<SetApprovalForAllResponse>> {
        let header_metadata = request.metadata();
        let contract_wallet = init_contract_wallet(header_metadata)
            .await
            .map_err(into_anyhow)?;

        let SetApprovalForAllRequest {
            contract,
            operator,
            approved,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let operator_address = operator
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, operator: {operator_address:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
        let calldata = contract
            .set_approval_for_all(operator_address, approved)
            .calldata()
            .ok_or(into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new().data(calldata);

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash
            .to_string();

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

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
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
        let contract_wallet = init_contract_wallet(header_metadata)
            .await
            .map_err(into_anyhow)?;

        let SafeTransferFromRequest {
            contract,
            from,
            to,
            token_id,
            value,
            data,
        } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let from = from.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let to = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_str(&token_id).map_err(|e| into_anyhow(e.into()))?;
        let value = parse_ether(value).map_err(|e| into_anyhow(e.into()))?;
        let data = hex::decode(data).map_err(|e| into_anyhow(e.into()))?.into();

        debug!("contract: {contract_address:?}, from: {from:?}, to: {to:?}, token_id: {token_id:?}, value: {value:?}, data: {data:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
        let calldata = contract
            .safe_transfer_from(from, to, token_id, value, data)
            .calldata()
            .ok_or(into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new().data(calldata);

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash
            .to_string();

        Ok(Response::new(SafeTransferFromResponse { txhash }))
    }

    async fn safe_batch_transfer_from(
        &self,
        request: Request<SafeBatchTransferFromRequest>,
    ) -> Result<Response<SafeBatchTransferFromResponse>> {
        let header_metadata = request.metadata();
        let contract_wallet = init_contract_wallet(header_metadata)
            .await
            .map_err(into_anyhow)?;

        let SafeBatchTransferFromRequest {
            contract,
            from,
            to,
            token_ids,
            values,
            data,
        } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let from = from.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let to = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let token_ids = token_ids
            .iter()
            .map(|t| U256::from_str(t).map_err(|e| into_anyhow(e.into())))
            .collect::<Result<Vec<_>>>()?;
        let values = values
            .iter()
            .map(|v| U256::from_str(v).map_err(|e| into_anyhow(e.into())))
            .collect::<Result<Vec<_>>>()?;
        let data = hex::decode(data).map_err(|e| into_anyhow(e.into()))?.into();

        debug!("contract: {contract_address:?}, from: {from:?}, to: {to:?}, token_ids: {token_ids:?}, values: {values:?}, data: {data:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
        let calldata = contract
            .safe_batch_transfer_from(from, to, token_ids, values, data)
            .calldata()
            .ok_or(into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new().data(calldata);

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash
            .to_string();

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
        let toke_id = U256::from_str(&token_id).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, account: {account:?}, token_id: {toke_id:?}");

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
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

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
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

        let contract = ERC404Contract::new(contract_address, Arc::clone(&self.client));
        let result = contract
            .erc_1155_transfer_exempt(target)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        Ok(Response::new(Erc1155TransferExemptResponse { result }))
    }
}
