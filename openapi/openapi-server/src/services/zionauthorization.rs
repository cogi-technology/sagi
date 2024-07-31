use {
    super::{
        reverted_error::*,
        utils::{into_anyhow, Result},
    },
    crate::{
        entity::telegram::{GetProofRequest, GetRequestType, GetSaltRequest, LoginWidgetData},
        helpers::{
            telegram::{authorize, get_init_data_integrity_web},
            utils::{send_request_json, send_request_text},
        },
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
    reqwest::{Client as ClientReqwest, Method},
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
        let mut salt: Response<String> = Response::new("".to_string());
        // let mut beneficiaries: Response<Vec<String>> = Response::new(vec!["".to_string()]);
        let mut beneficiaries: Response<String> = Response::new("".to_string());
        // Access a specific header, e.g., "authorization"
        if let Some(authorization_header) = metadata.get("authorization") {
            if let Ok(auth_str) = authorization_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    // Extract the JWT token by removing the "Bearer " prefix
                    let token = &auth_str["Bearer ".len()..];
                    println!("JWT Token: {}", token);
                    let client = ClientReqwest::new();
                    // Get Salt
                    let base_url_salt = env::var("NEXT_PUBLIC_SERVER_LOGIN_WITH_TELEGRAM")
                        .map_err(|e| into_anyhow(e.into()))?;
                    let url_salt = format!("{}/v1/salt", base_url_salt);
                    let body = GetSaltRequest {
                        jwt: token.to_string(),
                        index: 0,
                    };
                    salt = match send_request_text(&client, Method::POST, &url_salt, Some(&body), None)
                        .await
                    {
                        Ok(response) => response,
                        Err(e) => return Err(e),
                    };
                    // Get Proof
                    let url_proof = format!("{}/v1/prove", base_url_salt);
                    let body = GetProofRequest {
                        jwt: token.to_string(),
                        salt: salt.get_mut().to_string(),
                        signerPublicKey: token.to_string(),
                        keyClaimName: "sub".to_string(),
                        exp: 0,
                    };
                    salt = match send_request_text(&client, Method::POST, &url_salt, Some(&body), None)
                        .await
                    {
                        Ok(response) => response,
                        Err(e) => return Err(e),
                    };
                    // Get beneficiaries
                    let base_url_beneficiaries = env::var("NEXT_PUBLIC_TORII")
                        .map_err(|e| into_anyhow(e.into()))?;
                    let url_beneficiaries = format!("{}/v1/beneficiaries", base_url_beneficiaries);
                    beneficiaries = match send_request_text::<GetRequestType>(&client, Method::GET, &url_beneficiaries, None, None)
                        .await
                    {
                        Ok(response) => response,
                        Err(e) => return Err(e),
                    };
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
            salt: salt.get_mut().to_string(),
            proof: Some(proof_points), // Wrapping the proof_points in Some
            ephemeral_key_pair: "ephemeral_key".to_string(),
            beneficiaries: serde_json::from_str(beneficiaries.get_mut()).expect("beneficiariesRes could not be parsed")
        };
        Ok(Response::new(response))
    }
}
