use {
    super::{
        reverted_error::*,
        utils::{into_anyhow, Result},
        zionauthorization::get_data_request_for_zion_logic,
    },
    anyhow::anyhow,
    ethers::{
        types::{Address, BlockNumber},
        utils::parse_ether,
    },
    ethers_contract::{ContractError, ContractFactory},
    ethers_providers::Middleware,
    openapi_ethers::{
        client::Client as EthereumClient,
        erc20::{self as erc20_etherman, ERC20 as ERC20Contract, ERC20_ABI},
    },
    openapi_logger::debug,
    openapi_proto::erc20_service::{erc20_server::Erc20, *},
    std::{borrow::Borrow, sync::Arc},
    tonic::{Request, Response},
    zion_aa::{address_to_string, contract_wallet::wallet::ContractWallet},
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

        println!(
            "babalance: {:?}",
            self.client.get_balance(self.client.address(), None).await
        );

        let estimate_gas = self
            .client
            .provider()
            .estimate_gas(deployer.tx.borrow(), None)
            .await
            .map_err(|e| into_anyhow(e.into()))?;

        let block = self
            .client
            .provider()
            .get_block(BlockNumber::Latest)
            .await
            .map_err(|e| into_anyhow(e.into()))?
            .unwrap();
        let network_gas_limit = block.gas_limit;
        // Use the minimum of our calculated gas limit and the network gas limit
        let gas_limit = std::cmp::min(estimate_gas, network_gas_limit);

        deployer.tx.set_gas(1000000000);

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
        let metadata = req.metadata();
        let initial_data = get_data_request_for_zion_logic(metadata)
            .await
            .map_err(into_anyhow)?;

        let TotalSupplyRequest { contract } = req.into_inner();
        let contract_address = contract
            .parse::<Address>()
            .map_err(|e| into_anyhow(e.into()))?;

        let contract = ERC20Contract::new(contract_address, Arc::clone(&self.client));
        let calldata = contract
            .total_supply()
            .legacy()
            .calldata()
            .ok_or_else(|| into_anyhow(anyhow!("Functioncall convert to calldata failed")))?;

        // let contract_wallet = ContractWallet::new(contract_wallet_address, operator)

        Ok(Response::new(TotalSupplyResponse {
            total_supply: "total_supply".into(),
        }))
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
