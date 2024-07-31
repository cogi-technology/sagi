use {
    super::{
        reverted_error::*,
        utils::{into_anyhow, Result},
    },
    crate::{
        entity::telegram::LoginWidgetData,
        helpers::telegram::{authorize, get_init_data_integrity_web},
    },
    anyhow::anyhow,
    chrono::Utc,
    ethers::{types::Address, utils::parse_ether},
    ethers_contract::{ContractError, ContractFactory},
    grammers_client::{types::LoginToken, Client, Config, SignInError},
    grammers_session::Session,
    openapi_ethers::{
        client::Client as EthereumClient,
        erc20::{self as erc20_etherman, ERC20 as ERC20Contract, ERC20_ABI},
    },
    openapi_logger::debug,
    openapi_proto::zionauthorization_service::{zion_authorization_server::ZionAuthorization, *},
    serde_json::json,
    std::{env, fs, ptr::null, sync::Arc},
    tonic::{metadata::MetadataMap, Request, Response, Status},
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct ZionAuthorizationService {
    client: Arc<EthereumClient>,
}

impl ZionAuthorizationService {
    pub fn new(client: Arc<EthereumClient>) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl ZionAuthorization for ZionAuthorizationService {
    async fn get_data_request_for_zion(
        &self,
        req: Request<GetDataRequestForZionRequest>,
    ) -> Result<Response<GetDataRequestForZionResponse>> {
        let metadata: &MetadataMap = req.metadata();
        // Access a specific header, e.g., "authorization"
        if let Some(authorization_header) = metadata.get("authorization") {
            if let Ok(auth_str) = authorization_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    // Extract the JWT token by removing the "Bearer " prefix
                    let token = &auth_str["Bearer ".len()..];
                    println!("JWT Token: {}", token);

                    
                }
            } 
        } else {
            println!("No Authorization header found");
        }

        let proof_points = ProofPoints {
            protocol: "example_protocol".to_string(),
            pi_a: vec!["a".to_string(), "b".to_string()],
            pi_b: vec![
                StringArray {
                    values: vec!["x".to_string(), "y".to_string()],
                },
                StringArray {
                    values: vec!["z".to_string()],
                },
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
