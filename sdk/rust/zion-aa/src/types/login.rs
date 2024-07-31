use serde::Deserialize;

use super::jwt::ProofPoints;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginData {
    #[serde(deserialize_with = "deserialize_salt")]
    pub salt: String,
    pub proof: ProofPoints,
    pub ephemeral_key_pair: String,
    pub beneficiaries: Vec<String>,
}

fn deserialize_salt<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let ret = s.trim_start_matches("0x").to_string();
    Ok(ret)
}
