use {
    super::{
        reverted_error::*,
        utils::{into_anyhow, Result},
    },
    anyhow::anyhow,
    ethers::{types::Address, utils::parse_ether},
    ethers_contract::{ContractError, ContractFactory},
    openapi_ethers::{
        client::Client as EthereumClient,
        erc20::{self as erc20_etherman, ERC20 as ERC20Contract, ERC20_ABI},
    },
    openapi_logger::debug,
    openapi_proto::authtelegram_service::{auth_telegram_server::AuthTelegram, *},
    std::{ptr::null, sync::Arc},
    tonic::{Request, Response},
};

#[derive(Debug, Clone)]
pub struct AuthTelegramService {
    client: Arc<EthereumClient>,
}

impl AuthTelegramService {
    pub fn new(client: Arc<EthereumClient>) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl AuthTelegram for AuthTelegramService {
    async fn send_code_telegram(
        &self,
        req: Request<SendCodeTelegramRequest>,
    ) -> Result<Response<SendCodeTelegramResponse>> {
        Ok(Response::new(SendCodeTelegramResponse {
            phone_number: "".to_string(),
            phone_hash: "".to_string(),
        }))
    }
    async fn sign_in_telegram(
        &self,
        req: Request<SignInTelegramRequest>,
    ) -> Result<Response<SignInTelegramResponse>> {
        Ok(Response::new(SignInTelegramResponse {
            jwt: "".to_string(),
        }))
    }
    async fn log_out_telegram(
        &self,
        req: Request<LogOutTelegramRequest>,
    ) -> Result<Response<LogOutTelegramResponse>> {
        Ok(Response::new(LogOutTelegramResponse {
            phone_number: "".to_string(),
            phone_hash: "".to_string(),
        }))
    }
    async fn get_data_request_for_zion(
        &self,
        req: Request<GetDataRequestForZionRequest>,
    ) -> Result<Response<GetDataRequestForZionResponse>> {
        let proof_points = ProofPoints {
            protocol: "example_protocol".to_string(),
            pi_a: vec!["a".to_string(), "b".to_string()],
            pi_b: vec![
                StringArray { values: vec!["x".to_string(), "y".to_string()] },
                StringArray { values: vec!["z".to_string()] },
            ],
            pi_c: vec!["c1".to_string(), "c2".to_string()],
        };
        let response = GetDataRequestForZionResponse {
            salt: "some_salt".to_string(),
            proof: Some(proof_points), // Wrapping the proof_points in Some
            ephemeral_key_pair: "ephemeral_key".to_string(),
            beneficiaries: vec!["beneficiary1".to_string(), "beneficiary2".to_string()],
        };
        Ok(Response::new(response))
    }
}
