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
    ethers::{
        signers::{LocalWallet, Signer},
        types::Address,
        utils::parse_ether,
    },
    ethers_contract::{ContractError, ContractFactory},
    ethers_core::{k256::ecdsa::SigningKey, rand::rngs::OsRng},
    grammers_client::{types::LoginToken, Client, Config, SignInError},
    grammers_session::Session,
    hex,
    jsonwebtoken::TokenData,
    openapi_ethers::{
        client::Client as EthereumClient,
        erc20::{self as erc20_etherman, ERC20 as ERC20Contract, ERC20_ABI},
    },
    openapi_logger::debug,
    openapi_proto::zionauthorization_service::{zion_authorization_server::ZionAuthorization, *},
    reqwest::{Client as ClientReqwest, Method},
    serde_json::json,
    std::{env, fs, hash::Hash, ptr::null, sync::Arc},
    tonic::{metadata::MetadataMap, Request, Response, Status},
    uuid::Uuid,
    zion_aa::{contract_wallet::client::ClientMethods, types::jwt::JWTPayload},
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
        let mut salt: Response<String> = Response::new("".to_string());
        let mut proof: Response<zion_aa::types::jwt::ProofPoints> =
            Response::new(zion_aa::types::jwt::ProofPoints::default());
        let mut beneficiaries: Response<String> = Response::new("".to_string());
        // Wallet
        let signing_key = SigningKey::random(&mut OsRng).to_bytes();
        let ephemeral_key_pair = hex::encode(signing_key);
        let signer = LocalWallet::from_bytes(&signing_key).map_err(|e| into_anyhow(e.into()))?;
        let a = signer.address().0;
        let hex_string = hex::encode(a);
        // Access a specific header, e.g., "authorization"
        let metadata: &MetadataMap = req.metadata();
        if let Some(authorization_header) = metadata.get("authorization") {
            if let Ok(auth_str) = authorization_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    // Extract the JWT token by removing the "Bearer " prefix
                    let token: &str = &auth_str["Bearer ".len()..];
                    let parsed_token: TokenData<JWTPayload> =
                        zion_aa::utils::decode_jwt(token).unwrap();
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
                    salt = match send_request_text(
                        &client,
                        Method::POST,
                        &url_salt,
                        Some(&body),
                        None,
                    )
                    .await
                    {
                        Ok(response) => response,
                        Err(e) => return Err(e),
                    };

                    let url_proof = format!("{}/v1/prove", base_url_salt);
                    let body = GetProofRequest {
                        jwt: token.to_string(),
                        salt: salt.get_mut().to_string(),
                        signerPublicKey: format!("0x{}", hex_string),
                        keyClaimName: "sub".to_string(),
                        exp: parsed_token.claims.exp,
                    };
                    let proofT = client
                        .post(&url_proof)
                        .json(&body)
                        .send()
                        .await
                        .map_err(|e| into_anyhow(e.into()))?
                        .json::<zion_aa::types::jwt::ProofPoints>()
                        .await;
                    // .map_err(|e| into_anyhow(e.into()))?;
                    proof = match proofT {
                        Ok(response) => Response::new(response),
                        Err(e) => return Err(into_anyhow(e.into())),
                    };
                    // let proof_3 = match send_request_json::<ProofPoints, GetProofRequest>(&client, Method::POST, &url_salt, Some(&body), None)
                    //     .await
                    //     {
                    //         Ok(response) => response,
                    //         Err(e) => return Err(e),
                    //     };
                    // Get beneficiaries
                    let base_url_beneficiaries =
                        env::var("NEXT_PUBLIC_TORII").map_err(|e| into_anyhow(e.into()))?;
                    let url_beneficiaries = format!("{}/v1/beneficiaries", base_url_beneficiaries);
                    beneficiaries = match send_request_text::<GetRequestType>(
                        &client,
                        Method::GET,
                        &url_beneficiaries,
                        None,
                        None,
                    )
                    .await
                    {
                        Ok(response) => response,
                        Err(e) => return Err(e),
                    };
                }
            }
        }
        // Response
        let proofJson: &zion_aa::types::jwt::ProofPoints = proof.get_mut();
        let pi_bRes = proofJson
            .pi_b
            .iter()
            .map(|x| StringArray { values: x.clone() })
            .collect();
        let proof_points = ProofPoints {
            pi_a: proofJson.pi_a.clone(),
            pi_b: pi_bRes,
            pi_c: proofJson.pi_c.clone(),
            protocol: proofJson.protocol.clone().expect("REASON"),
        };
        let response = GetDataRequestForZionResponse {
            salt: salt.get_mut().to_string(),
            proof: Some(proof_points),
            ephemeral_key_pair: format!("0x{}", ephemeral_key_pair),
            beneficiaries: serde_json::from_str(beneficiaries.get_mut())
                .expect("beneficiariesRes could not be parsed"),
        };
        Ok(Response::new(response))
    }
}
