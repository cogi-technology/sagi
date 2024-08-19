use {
    serde::{Deserialize, Serialize},
    std::{default::Default, fmt::Debug},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JwkKeyPairAlg {
    RS256,
    RS384,
    RS512,
    EdDSA,
}

impl Default for JwkKeyPairAlg {
    fn default() -> Self {
        Self::RS256
    }
}

impl From<String> for JwkKeyPairAlg {
    fn from(value: String) -> Self {
        match value.as_str() {
            "RS256" => JwkKeyPairAlg::RS256,
            "RS384" => JwkKeyPairAlg::RS384,
            "RS512" => JwkKeyPairAlg::RS512,
            "EdDSA" => JwkKeyPairAlg::EdDSA,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)] // must be uppercase by definition
pub enum JwkKeyPairType {
    RSA,
    OKP,
}

impl Default for JwkKeyPairType {
    fn default() -> Self {
        Self::RSA
    }
}

impl JwkKeyPairType {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match self {
            JwkKeyPairType::RSA => "RSA",
            JwkKeyPairType::OKP => "OKP",
        }
    }
}
