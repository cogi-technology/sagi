use {
    super::utils::{into_anyhow, Result},
    ethers::{types::Address, utils::parse_ether},
    ethers_contract::ContractFactory,
    openapi_ethers::{
        erc20::{self as erc20_etherman, ERC20 as ERC20Contract, ERC20_ABI},
        client::Client as EthereumClient,
    },
    openapi_proto::erc20_service::{erc20_server::Erc20, *},
    std::sync::Arc,
    tonic::{Request, Response},
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

        let factory = ContractFactory::new(
            ERC20_ABI.clone(),
            erc20_etherman::erc20_bytecode().into(),
            Arc::clone(&self.client),
        );

        let contract = factory
            .deploy((owner, name, symbol, initial_supply))
            .map_err(|e| into_anyhow(e.into()))?
            .send()
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        Ok(Response::new(DeployResponse {
            contract: contract.address().to_string(),
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
        let function_call = contract.approve(spender_address, amount).legacy();
        let txhash = function_call
            .send()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .tx_hash()
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
            .call()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .to_string();

        Ok(Response::new(AllowanceResponse { remaining_amount }))
    }

    async fn transfer(&self, req: Request<TransferRequest>) -> Result<Response<TransferResponse>> {
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
        let function_call = contract.transfer(recipient_address, amount).legacy();
        let txhash = function_call
            .send()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .tx_hash()
            .to_string();

        Ok(Response::new(TransferResponse { txhash }))
    }

    async fn transfer_from(
        &self,
        req: Request<TransferFromRequest>,
    ) -> Result<Response<TransferFromResponse>> {
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
        let function_call = contract
            .transfer_from(sender_address, recipient_address, amount)
            .legacy();
        let txhash = function_call
            .send()
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .tx_hash()
            .to_string();

        Ok(Response::new(TransferFromResponse { txhash }))
    }
}
