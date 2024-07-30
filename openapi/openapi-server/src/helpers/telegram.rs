use {
    super::utils::into_anyhow, crate::entity::telegram::{AuthRequest, AuthResponse, LoginWidgetData}, hex::encode, hmac::{Hmac, Mac}, reqwest::Client, serde_json::json, sha2::{Digest, Sha256}, std::env
};

// Define a type alias for Hmac-Sha256
type HmacSha256 = Hmac<Sha256>;

pub fn get_init_data_integrity_web(
    init_data_unsafe: &LoginWidgetData,
    telegram_bot_token: &str,
) -> LoginWidgetData {
    // Hash the telegram bot token to create a secret
    let mut hasher = Sha256::new();
    hasher.update(&telegram_bot_token);
    let secret = hasher.finalize();
    // Convert result to hexadecimal string
    let data_to_hash = format!(
        "auth_date={}\nfirst_name={}\nid={}\nlast_name={}\nphoto_url={}\nusername={}",
        init_data_unsafe.auth_date,
        init_data_unsafe.first_name.to_string(),
        init_data_unsafe.id,
        init_data_unsafe.last_name.as_deref().unwrap_or(""),
        init_data_unsafe.photo_url.as_deref().unwrap_or(""),
        init_data_unsafe.username.as_deref().unwrap_or("")
    );
    // Compute the HMAC
    let mut mac = HmacSha256::new_from_slice(&secret).expect("HMAC can take key of any size");
    mac.update(data_to_hash.as_bytes());
    let hash_result = mac.finalize();
    let hash_bytes = hash_result.into_bytes();

    // Convert the HMAC result to a hexadecimal string
    let hash_hex: String = hex::encode(hash_bytes);
    let mut result: LoginWidgetData = init_data_unsafe.clone();
    result.hash = Some(hash_hex);
    return result;
}


pub async fn authorize(base_url: &str, client_id: &str, init_data: LoginWidgetData) -> Result<AuthResponse, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/auth/v1/oidc/authorize", base_url);

    let auth_request = AuthRequest {
        client_id: client_id.to_string(),
        init_data,
    };

    let response = client
        .post(&url)
        .json(&auth_request)
        .send()
        .await?
        .json::<AuthResponse>()
        .await?;

    Ok(response)
}
