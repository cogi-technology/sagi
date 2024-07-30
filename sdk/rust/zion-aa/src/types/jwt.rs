use ethers::types::U256;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, vec::Vec};

// Structs corresponding to TypeScript interfaces

#[derive(Clone, Deserialize, Serialize)]
pub struct ProofPoints {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct JWTHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JWTPayload {
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: u64, // Optional integer fields
    pub exp: u64, // Assuming number is a 64-bit integer for time
    pub nbf: u64,
    pub iss: String,
    pub sub: String,
    pub aud: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub at_hash: Option<String>,
}

#[derive(Clone)]
pub struct JWTOptions<S> {
    pub header: JWTHeader,
    pub payload: JWTPayload,
    pub proof: ProofPoints,
    pub ephemeral_key_pair: Arc<S>,
    pub deadline: U256,
    pub salt: String,
}
