use {
    super::utils::{init_contract_wallet, into_anyhow, Result},
    anyhow::anyhow,
    ethers::{
        types::{Address, BlockNumber, Eip1559TransactionRequest, TransactionRequest, U256},
        utils::parse_ether,
    },
    ethers_contract::ContractFactory,
    ethers_providers::Middleware,
    openapi_ethers::{
        client::Client as EthereumClient,
        erc20::{self as erc20_etherman, ERC20 as ERC20Contract, ERC20_ABI},
    },
    openapi_logger::debug,
    openapi_proto::erc20_service::{erc20_server::Erc20, *},
    std::{borrow::Borrow, sync::Arc},
    tonic::{Request, Response},
    zion_aa::address_to_string,
};

#[derive(Debug, Clone)]
pub struct Erc20Service {
    client: Arc<EthereumClient>,
}

impl Erc20Service {
    pub fn new(client: Arc<EthereumClient>) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl Erc20 for Erc20Service {
    async fn deploy(&self, req: Request<DeployRequest>) -> Result<Response<DeployResponse>> {
        debug!("{req:?}");
        let DeployRequest {
            owner,
            name,
            symbol,
            initial_supply,
        } = req.into_inner();
        let owner = owner
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let initial_supply = parse_ether(initial_supply).map_err(|e| into_anyhow(e.into()))?;

        debug!("owner: {owner:?}, initial_supply: {initial_supply:?}");

        let factory = ContractFactory::new(
            ERC20_ABI.clone(),
            erc20_etherman::erc20_bytecode().into(),
            Arc::clone(&self.client),
        );

        let mut deployer = factory
            .deploy((name, symbol, initial_supply))
            .map_err(|e| into_anyhow(e.into()))?;

        deployer.tx.set_gas(U256::from(1_000_000));
        deployer.tx.set_gas_price(U256::from(10_000));

        let deployed_contract = deployer
            .legacy()
            .send()
            .await
            .map_err(|e| into_anyhow(e.into()))?;
        let contract_address = address_to_string!(deployed_contract.address());

        debug!("contract address: {}", contract_address);

        Ok(Response::new(DeployResponse {
            contract: contract_address,
        }))
    }

    async fn total_supply(
        &self,
        req: Request<TotalSupplyRequest>,
    ) -> Result<Response<TotalSupplyResponse>> {
        let TotalSupplyRequest { contract } = req.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.client));
        let total_supply = contract
            .total_supply()
            .legacy()
            .call()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        Ok(Response::new(TotalSupplyResponse { total_supply }))
    }

    async fn approve(&self, req: Request<ApproveRequest>) -> Result<Response<ApproveResponse>> {
        let header_metadata = req.metadata();
        let contract_wallet = init_contract_wallet(header_metadata)
            .await
            .map_err(into_anyhow)?;

        let ApproveRequest {
            contract,
            spender,
            amount,
        } = req.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let spender_address = spender
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let amount = parse_ether(amount).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.client));
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
        req: Request<BalanceOfRequest>,
    ) -> Result<Response<BalanceOfResponse>> {
        let BalanceOfRequest { contract, account } = req.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let account_address = account
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.client));
        let amount = contract
            .balance_of(account_address)
            .legacy()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        Ok(Response::new(BalanceOfResponse { amount }))
    }

    async fn allowance(
        &self,
        req: Request<AllowanceRequest>,
    ) -> Result<Response<AllowanceResponse>> {
        let AllowanceRequest {
            contract,
            owner,
            spender,
        } = req.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let owner_address = owner
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let spender_address = spender
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.client));
        let remaining_amount = contract
            .allowance(owner_address, spender_address)
            .legacy()
            .call()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        Ok(Response::new(AllowanceResponse { remaining_amount }))
    }

    async fn transfer(&self, req: Request<TransferRequest>) -> Result<Response<TransferResponse>> {
        let contract_wallet = init_contract_wallet(req.metadata())
            .await
            .map_err(into_anyhow)?;

        let TransferRequest {
            contract,
            recipient,
            amount,
        } = req.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let recipient_address = recipient
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;
        let amount = parse_ether(amount).map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.client));
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
        req: Request<TransferFromRequest>,
    ) -> Result<Response<TransferFromResponse>> {
        let contract_wallet = init_contract_wallet(req.metadata())
            .await
            .map_err(into_anyhow)?;

        let TransferFromRequest {
            contract,
            sender,
            recipient,
            amount,
        } = req.into_inner();
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

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.client));
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
}
