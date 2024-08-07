use crate::types::jwt::ProofPoints;
use serde::Deserialize;

use super::proof::{sdk_proofpoint_from, RequestProofPoints};

#[derive(Debug, Clone, Deserialize)]
pub struct AuthorizationData {
    #[serde(deserialize_with = "deserialize_salt")]
    pub salt: String,
    #[serde(deserialize_with = "deserialize_proof")]
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

fn deserialize_proof<'de, D>(deserializer: D) -> Result<ProofPoints, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let proof: RequestProofPoints = Deserialize::deserialize(deserializer)?;
    let ret = sdk_proofpoint_from(proof);
    Ok(ret)
}
