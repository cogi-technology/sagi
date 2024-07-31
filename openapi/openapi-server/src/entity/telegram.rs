use serde::{Deserialize, Serialize};

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
    pub hash: Option<String>
}


#[derive(Serialize)]
pub struct AuthRequest {
    pub client_id: String,
    pub init_data: LoginWidgetData,
}

#[derive(Deserialize)]
pub struct AuthResponse {
    // Define the response fields according to your API
    pub id_token: Option<String>,
    pub error: Option<String>,
    pub message: Option<String>,
    pub timestamp: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct GetSaltRequest {
    pub jwt: String,
    pub index: i32,
}

pub struct GetProofRequest {
    pub jwt: String,
    pub salt: String,
    pub signerPublicKey: String,
    pub exp: i64,
    pub keyClaimName: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetRequestType {
}