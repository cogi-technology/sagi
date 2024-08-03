use {
    super::utils::{init_contract_wallet, Result},
    crate::helpers::utils::into_anyhow,
    anyhow::anyhow,
    ethers::types::{Address, Eip1559TransactionRequest, U256},
    ethers_contract::ContractFactory,
    openapi_ethers::{
        client::Client as EthereumClient,
        erc721::{erc721_bytecode, ERC721 as ERC721Contract, ERC721_ABI},
    },
    openapi_logger::debug,
    openapi_proto::erc721_service::{erc721_server::Erc721, *},
    std::{str::FromStr, sync::Arc},
    tonic::{Request, Response},
    zion_aa::address_to_string,
};

pub struct Erc721Service {
    client: Arc<EthereumClient>,
}

impl Erc721Service {
    pub fn new(client: Arc<EthereumClient>) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl Erc721 for Erc721Service {
    async fn deploy(&self, request: Request<DeployRequest>) -> Result<Response<DeployResponse>> {
        debug!("{request:?}");

        let DeployRequest {
            name,
            symbol,
            owner,
        } = request.into_inner();

        let owner = owner
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        debug!("owner: {owner:?}");

        let factory = ContractFactory::new(
            ERC721_ABI.clone(),
            erc721_bytecode(),
            Arc::clone(&self.client),
        );

        let contract = factory
            .deploy((name, symbol))
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

    async fn balance_of(
        &self,
        request: Request<BalanceOfRequest>,
    ) -> Result<Response<BalanceOfResponse>> {
        let BalanceOfRequest { contract, owner } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let owner_address = owner
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        debug!(
            "contract address: {:?}, owner address: {:?}",
            contract_address, owner_address
        );

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.client));
        let amount = contract
            .balance_of(owner_address)
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

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.client));
        let owner = contract
            .owner_of(U256::from_str(token_id.as_str()).map_err(|e| into_anyhow(e.into()))?)
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
        let contract_wallet = init_contract_wallet(header_metadata)
            .await
            .map_err(into_anyhow)?;

        let SafeTransferFromRequest {
            contract,
            from,
            to,
            token_id,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let from_address = from.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let to_address = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_str(&token_id).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, from: {from_address:?}, to: {to_address:?}, token_id: {token_id:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.client));
        let calldata = contract
            .safe_transfer_from(from_address, to_address, token_id)
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

    async fn transfer_from(
        &self,
        request: Request<TransferFromRequest>,
    ) -> Result<Response<TransferFromResponse>> {
        let header_metadata = request.metadata();
        let contract_wallet = init_contract_wallet(header_metadata)
            .await
            .map_err(into_anyhow)?;

        let TransferFromRequest {
            contract,
            from,
            to,
            token_id,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let from_address = from.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let to_address = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_str(&token_id).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, from: {from_address:?}, to: {to_address:?}, token_id: {token_id:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.client));
        let calldata = contract
            .transfer_from(from_address, to_address, token_id)
            .calldata()
            .ok_or(into_anyhow(anyhow!("Calldata is None")))?;
        let transaction = Eip1559TransactionRequest::new().data(calldata);

        let txhash = contract_wallet
            .send_transaction(transaction, None)
            .await
            .map_err(into_anyhow)?
            .transaction_hash
            .to_string();

        Ok(Response::new(TransferFromResponse { txhash }))
    }

    async fn approve(&self, request: Request<ApproveRequest>) -> Result<Response<ApproveResponse>> {
        let header_metadata = request.metadata();
        let contract_wallet = init_contract_wallet(header_metadata)
            .await
            .map_err(into_anyhow)?;

        let ApproveRequest {
            contract,
            to,
            token_id,
        } = request.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let to_address = to.parse::<Address>().map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_str(&token_id).map_err(|e| into_anyhow(e.into()))?;

        debug!("contract: {contract_address:?}, to: {to_address:?}, token_id: {token_id:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.client));
        let calldata = contract
            .approve(to_address, token_id)
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

    async fn get_approved(
        &self,
        request: Request<GetApprovedRequest>,
    ) -> Result<Response<GetApprovedResponse>> {
        let GetApprovedRequest { contract, token_id } = request.into_inner();

        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let token_id = U256::from_str(&token_id).map_err(|e| into_anyhow(e.into()))?;
        debug!("contract: {contract_address:?}, token_id: {token_id:?}");

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.client));
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

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.client));
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

        let contract = ERC721Contract::new(contract_address, Arc::clone(&self.client));
        let ret = contract
            .is_approved_for_all(owner_adress, operator_address)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        Ok(Response::new(IsApprovedForAllResponse { result: ret }))
    }
}
