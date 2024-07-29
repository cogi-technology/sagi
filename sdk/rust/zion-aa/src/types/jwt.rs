use ethers::types::U256;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, vec::Vec};

// Structs corresponding to TypeScript interfaces

#[derive(Clone, Deserialize, Serialize)]
pub struct ProofPoints {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    pub protocol: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct JWTHeader {
    pub alg: Option<String>,
    pub typ: Option<String>,
    pub kid: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct JWTPayload {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub exp: u64,         // Assuming number is a 64-bit integer for time
    pub iat: Option<u64>, // Optional integer fields
    pub at_hash: Option<String>,
}

#[derive(Clone)]
pub struct JWTOptions<S> {
    pub header: JWTHeader,
    pub payload: JWTPayload,
    pub proof: ProofPoints,
    pub ephemeral_key_pair: Arc<S>,
    pub deadline: U256,
    pub salt: [u8; 32],
}
