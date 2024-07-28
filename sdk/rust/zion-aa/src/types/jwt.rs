use ethers::signers::Signer;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

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

#[derive(Clone, Deserialize, Serialize)]
pub struct JWTOptions<S: Signer + 'static> {
    pub header: JWTHeader,
    pub payload: JWTPayload,
    pub proof: ProofPoints,
    pub ephemeral_key_pair: S, // This will need to be handled appropriately in Rust
    pub deadline: u64,
    pub salt: String,
}
