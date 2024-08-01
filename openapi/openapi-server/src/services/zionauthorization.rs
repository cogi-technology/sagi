use {
    super::utils::{into_anyhow, Result as TonicResult},
    crate::{
        entity::telegram::{GetProofRequest, GetRequestType, GetSaltRequest},
        helpers::{into::proto_proofpoint_from, utils::send_request_text},
    },
    anyhow::{anyhow, Result},
    ethers::signers::{LocalWallet, Signer},
    ethers_core::{k256::ecdsa::SigningKey, rand::rngs::OsRng},
    jsonwebtoken::TokenData,
    openapi_ethers::client::Client as EthereumClient,
    openapi_logger::debug,
    openapi_proto::zionauthorization_service::{zion_authorization_server::ZionAuthorization, *},
    reqwest::{Client as ClientReqwest, Method},
    std::{env, sync::Arc},
    tonic::{metadata::MetadataMap, Request, Response},
    zion_aa::{
        address_to_string,
        types::jwt::{JWTPayload, ProofPoints as SdkProofPoints},
    },
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
    ) -> TonicResult<Response<GetDataRequestForZionResponse>> {
        let metadata = req.metadata();
        let (response, _) = get_data_request_for_zion_logic(metadata)
            .await
            .map_err(into_anyhow)?;

        Ok(Response::new(response))
    }
}

pub async fn get_data_request_for_zion_logic(
    metadata: &MetadataMap,
) -> Result<(GetDataRequestForZionResponse, TokenData<JWTPayload>)> {
    // Wallet
    // Generate a random ephemeral_key_pair
    let signing_key = SigningKey::random(&mut OsRng).to_bytes();
    let signer = LocalWallet::from_bytes(&signing_key).map_err(|e| into_anyhow(e.into()))?;
    let ephemeral_key_pair = hex::encode(signing_key);
    let signer_public_key = address_to_string!(signer.address());

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
    debug!("parsed_token: {:?}", parsed_token);

    // Get Login data
    let client = ClientReqwest::new();
    let base_url = env::var("NEXT_PUBLIC_SERVER_LOGIN_WITH_TELEGRAM")?;

    // get salt
    let base_url_salt = env::var("NEXT_PUBLIC_SERVER_LOGIN_WITH_TELEGRAM")?;
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
    let sdk_proof = client
        .post(&url_proof)
        .json(&body)
        .send()
        .await?
        .json::<SdkProofPoints>()
        .await?;

    // Get beneficiaries
    let base_url_beneficiaries = env::var("NEXT_PUBLIC_TORII")?;
    let url_beneficiaries = format!("{}/v1/beneficiaries", base_url_beneficiaries);
    let beneficiaries =
        send_request_text::<GetRequestType>(&client, Method::GET, &url_beneficiaries, None, None)
            .await?
            .into_inner();

    // Response
    let proto_proof = proto_proofpoint_from(sdk_proof);

    let response = GetDataRequestForZionResponse {
        salt,
        proof: Some(proto_proof),
        ephemeral_key_pair,
        beneficiaries: serde_json::from_str(&beneficiaries)?,
    };

    Ok((response, parsed_token))
}
