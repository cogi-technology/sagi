use {
    crate::helpers::jwk::{JwkKeyPairAlg, JwkKeyPairType},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginWidgetData {
    pub id: i64,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,

    pub auth_date: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Serialize)]
pub struct AuthRequest {
    pub client_id: String,
    pub init_data: LoginWidgetData,
    // pub session_uuid: String,
}

#[derive(Deserialize)]
pub struct AuthResponse {
    // Define the response fields according to your API
    pub id_token: Option<String>,
    pub error: Option<String>,
    #[allow(dead_code)]
    pub message: Option<String>,
    #[allow(dead_code)]
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct GetSaltRequest {
    pub jwt: String,
    pub index: i32,
}
#[derive(Serialize, Deserialize)]
pub struct GetProofRequest {
    pub jwt: String,
    pub salt: String,
    #[serde(rename = "signerPublicKey")]
    pub signer_public_key: String,
    pub exp: u64,
    #[serde(rename = "keyClaimName")]
    pub key_claim_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRequestType {
    pub keys: Vec<JWKSPublicKeyCerts>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWKSPublicKeyCerts {
    pub kty: JwkKeyPairType,
    pub alg: JwkKeyPairAlg,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>, // Ed25519
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>, // RSA
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<String>, // RSA
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>, // OCT
}
