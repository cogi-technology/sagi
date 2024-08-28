use {
    crate::{
        cache::JWT_CACHE,
        config::TelegramAuthConfig,
        entity::telegram::{GetProofRequest, GetRequestType, GetSaltRequest},
        error::{into_anyhow, Result as TonicResult},
        helpers::into::proto_proofpoint_from,
    },
    anyhow::{anyhow, Result},
    ethers::signers::{LocalWallet, Signer},
    ethers_core::{k256::ecdsa::SigningKey, rand::rngs::OsRng},
    jsonwebtoken::TokenData,
    openapi_logger::debug,
    openapi_proto::zionauthorization_service::{zion_authorization_server::ZionAuthorization, *},
    reqwest::{Client as ClientReqwest, Method},
    tonic::{metadata::MetadataMap, Request, Response},
    zion_aa::{
        address_to_string,
        types::{
            jwt::{JWTPayload, ProofPoints as SdkProofPoints},
            request::AuthorizationData,
        },
    },
    webhook_etherman::utils::send_request_text,
};

#[derive(Debug, Clone)]
pub struct ZionAuthorizationService {
    pub cfg: TelegramAuthConfig,
}

impl ZionAuthorizationService {
    pub fn new(telegram_auth: TelegramAuthConfig) -> Self {
        Self { cfg: telegram_auth }
    }
}

#[tonic::async_trait]
impl ZionAuthorization for ZionAuthorizationService {
    async fn get_data_request_for_zion(
        &self,
        req: Request<GetDataRequestForZionRequest>,
    ) -> TonicResult<Response<GetDataRequestForZionResponse>> {
        let metadata = req.metadata();
        let (
            AuthorizationData {
                salt,
                proof,
                ephemeral_key_pair,
                beneficiaries,
            },
            _,
        ) = get_data_request_for_zion_logic(metadata, &self.cfg)
            .await
            .map_err(into_anyhow)?;

        let proof = proto_proofpoint_from(proof);
        let response = GetDataRequestForZionResponse {
            salt,
            proof: Some(proof),
            ephemeral_key_pair,
            beneficiaries,
        };

        Ok(Response::new(response))
    }
}

pub async fn get_data_request_for_zion_logic(
    metadata: &MetadataMap,
    config: &TelegramAuthConfig,
) -> Result<(AuthorizationData, TokenData<JWTPayload>)> {
    // Access a specific header, e.g., "authorization"
    let authorization_header = metadata
        .get("authorization")
        .ok_or(anyhow!("Authorization header not found"))?
        .to_str()?;
    if !authorization_header.starts_with("Bearer ") {
        return Err(anyhow!("Invalid authorization header"));
    }

    // Extract the JWT token by removing the "Bearer " prefix
    let token = &authorization_header["Bearer ".len()..];
    let parsed_token = zion_aa::utils::decode_jwt(token)?;

    if let Some((data, _)) = JWT_CACHE.read().get(token) {
        debug!("token already in cache");
        return Ok((data.clone(), parsed_token));
    }

    debug!("token not in cache");
    // Wallet
    // Generate a random ephemeral_key_pair
    let signing_key = SigningKey::random(&mut OsRng).to_bytes();
    let signer = LocalWallet::from_bytes(&signing_key).map_err(|e| into_anyhow(e.into()))?;
    let ephemeral_key_pair = hex::encode(signing_key);
    let signer_public_key = address_to_string!(signer.address());

    // Get Login data
    let client = ClientReqwest::new();
    let base_url = config.next_public_server_login_with_telegram.clone();

    // get salt
    let base_url_salt = config.next_public_server_login_with_telegram.clone();
    let url_salt = format!("{}/v1/salt", base_url_salt);
    let body = GetSaltRequest {
        jwt: token.to_string(),
        index: 0,
    };
    let salt = send_request_text(&client, Method::POST, &url_salt, Some(&body), None)
        .await?
        .into_inner();

    // get proof
    let url_proof = format!("{}/v1/prove", base_url);
    let body = GetProofRequest {
        jwt: token.to_string(),
        salt: salt.clone(),
        signer_public_key,
        key_claim_name: "sub".to_string(),
        exp: parsed_token.claims.exp,
    };
    let proof = client
        .post(&url_proof)
        .json(&body)
        .send()
        .await?
        .json::<SdkProofPoints>()
        .await?;

    // Get beneficiaries
    let base_url_beneficiaries = config.next_public_torii.clone();
    let url_beneficiaries = format!("{}/v1/beneficiaries", base_url_beneficiaries);
    let beneficiaries =
        send_request_text::<GetRequestType>(&client, Method::GET, &url_beneficiaries, None, None)
            .await?
            .into_inner();

    let salt = salt.trim_start_matches("0x").to_string();
    // Response
    let data = AuthorizationData {
        salt,
        proof,
        ephemeral_key_pair,
        beneficiaries: serde_json::from_str(&beneficiaries)?,
    };

    JWT_CACHE
        .write()
        // .insert(token.to_string(), (data.clone(), parsed_token.claims.exp));
        .insert(token.to_string(), (data.clone(), parsed_token.claims.exp));

    Ok((data, parsed_token))
}
