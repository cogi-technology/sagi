use {
    anyhow::Result,
    ethers::types::U256,
    jsonwebtoken::{Header, TokenData},
    serde::{Deserialize, Serialize},
    std::{str::FromStr, sync::Arc, vec::Vec},
};

// Structs corresponding to TypeScript interfaces

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProofPoints {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

impl Default for ProofPoints {
    fn default() -> Self {
        ProofPoints {
            pi_a: vec!["".to_string()],
            pi_b: vec![vec!["".to_string()]],
            pi_c: vec!["".to_string()],
            protocol: None,
        }
    }
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
    pub fn try_init(
        token_data: TokenData<JWTPayload>,
        ephemeral_key_pair: String,
        proof: ProofPoints,
        salt: String,
    ) -> Result<Self>
    where
        S: FromStr,
        <S as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        let deadline = token_data.claims.exp;
        let ephemeral_key_pair = ephemeral_key_pair.parse()?;

        Ok(Self {
            header: token_data.header,
            payload: token_data.claims,
            proof,
            ephemeral_key_pair: Arc::new(ephemeral_key_pair),
            deadline: deadline.into(),
            salt,
        })
    }
}
