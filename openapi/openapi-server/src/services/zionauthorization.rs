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
    debug!(
        "get_data_request_for_zion_logic::authorization_header: {}",
        authorization_header
    );

    // Extract the JWT token by removing the "Bearer " prefix
    let token = &authorization_header["Bearer ".len()..];
    let parsed_token = zion_aa::utils::decode_jwt(token)?;
    debug!(
        "get_data_request_for_zion_logic::parsed_token: {:?}",
        parsed_token
    );

    // Get Login data
    let client = ClientReqwest::new();
    let base_url = env::var("NEXT_PUBLIC_SERVER_LOGIN_WITH_TELEGRAM")?;
    debug!("get_data_request_for_zion_logic::base_url: {:?}", base_url);

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
    debug!("get_data_request_for_zion_logic::salt: {}", salt);

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
    debug!(
        "get_data_request_for_zion_logic::sdk_proof: {:#?}",
        sdk_proof
    );
    // Proof for Response
    let proto_proof = proto_proofpoint_from(sdk_proof);
    debug!("get_data_request_for_zion_logic::proto_proof: {:#?}", proto_proof);

    // Get beneficiaries

    let base_url_beneficiaries = env::var("NEXT_PUBLIC_TORII")?;
    let url_beneficiaries = format!("{}/v1/beneficiaries", base_url_beneficiaries);
    let beneficiaries =
        send_request_text::<GetRequestType>(&client, Method::GET, &url_beneficiaries, None, None)
            .await?
            .into_inner();
    debug!("get_data_request_for_zion_logic::beneficiaries: {}", beneficiaries);

    // Response
    let response = GetDataRequestForZionResponse {
        salt,
        proof: Some(proto_proof),
        ephemeral_key_pair,
        beneficiaries: serde_json::from_str(&beneficiaries)?,
    };
    debug!("get_data_request_for_zion_logic::response: {:#?}", response);

    Ok((response, parsed_token))
}
