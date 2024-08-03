use {
    super::utils::{into_anyhow, Result},
    crate::{
        config::{Cli, TelegramAuthConfig, Config as ConfigPro},
        entity::telegram::{GetProofRequest, GetRequestType, GetSaltRequest},
        helpers::utils::send_request_text,
    },
    anyhow::anyhow,
    clap::Parser,
    ethers::signers::{LocalWallet, Signer},
    ethers_core::{k256::ecdsa::SigningKey, rand::rngs::OsRng},
    hex,
    jsonwebtoken::TokenData,
    openapi_proto::zionauthorization_service::{zion_authorization_server::ZionAuthorization, *},
    reqwest::{Client as ClientReqwest, Method},
    std::fs,
    tonic::{metadata::MetadataMap, Request, Response},
    zion_aa::types::jwt::JWTPayload,
};

#[derive(Debug, Clone)]
pub struct ZionAuthorizationService {
    pub cfg: TelegramAuthConfig,
}

impl ZionAuthorizationService {
    pub fn new() -> Self {
        let args = Cli::parse();
        let cfg = fs::read_to_string(args.cfg.clone())
            .map_err(|x| anyhow!("Failed {} {}", args.cfg, x.to_string()))
            .unwrap();
        let c = ConfigPro::from_cfg(cfg.as_str())
            .map_err(|x| anyhow!("Failed {} {}", args.cfg, x.to_string()))
            .unwrap();

        Self { cfg: c.telegram_auth }
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
                    let client = ClientReqwest::new();
                    // Get Salt
                    let base_url_salt = self.cfg.next_public_server_login_with_telegram.clone();
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
                    let proof_data = client
                        .post(&url_proof)
                        .json(&body)
                        .send()
                        .await
                        .map_err(|e| into_anyhow(e.into()))?
                        .json::<zion_aa::types::jwt::ProofPoints>()
                        .await;
                    // .map_err(|e| into_anyhow(e.into()))?;
                    proof = match proof_data {
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
                    let base_url_beneficiaries = self.cfg.next_public_torii.clone();
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
        let proof_point_data: &zion_aa::types::jwt::ProofPoints = proof.get_mut();
        let pi_b_res = proof_point_data
            .pi_b
            .iter()
            .map(|x| StringArray { values: x.clone() })
            .collect();
        let proof_points = ProofPoints {
            pi_a: proof_point_data.pi_a.clone(),
            pi_b: pi_b_res,
            pi_c: proof_point_data.pi_c.clone(),
            protocol: proof_point_data.protocol.clone().expect("protocol"),
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
