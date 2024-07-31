use super::login::LoginData;
use anyhow::Result;
use ethers::{signers::WalletError, types::U256};
use jsonwebtoken::{Header, TokenData};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc, vec::Vec};

// Structs corresponding to TypeScript interfaces

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ProofPoints {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

pub type JWTHeader = Header;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct JWTPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<u64>,
    pub exp: u64,
    pub nbf: u64,
    pub iss: String,
    pub sub: String,
    pub aud: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_hash: Option<String>,
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

impl<S: FromStr> JWTOptions<S> {
    pub fn try_into(token_data: TokenData<JWTPayload>, login_data: LoginData) -> Result<Self>
    where
        S: FromStr<Err = WalletError>,
    {
        let deadline = token_data.claims.exp;
        let ephemeral_key_pair = login_data.ephemeral_key_pair.parse()?;

        Ok(Self {
            header: token_data.header,
            payload: token_data.claims,
            proof: login_data.proof,
            ephemeral_key_pair: Arc::new(ephemeral_key_pair),
            deadline: deadline.into(),
            salt: login_data.salt,
        })
    }
}
