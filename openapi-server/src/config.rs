use {
    anyhow::{anyhow, Result},
    clap::Parser,
    openapi_ethers::config::Config as EthereumClientConfig,
    serde::{Deserialize, Serialize},
    std::{fs, net::SocketAddr},
    tonic::transport::Identity,
    webhook_etherman::config::Config as EthereumConfig,
};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Config {
    #[serde(rename = "grpc-listen")]
    pub grpc_listen: SocketAddr,

    #[serde(rename = "openapi-listen")]
    pub openapi_listen: SocketAddr,

    #[serde(rename = "rpc-client")]
    pub rpc_client: EthereumClientConfig,

    pub tls: TlsConfig,

    #[serde(rename = "telegram-auth")]
    pub telegram_auth: TelegramAuthConfig,

    #[serde(rename = "private-key-path")]
    pub private_key_path: String,

    #[serde(rename = "auth-secret")]
    pub auth_secret: String,

    #[serde(rename = "doc-path")]
    pub doc_path: String,

    #[serde(rename = "db-url")]
    pub db_url: String,

    pub etherman: EthereumConfig,
}

impl Config {
    pub fn from_cfg(cfg: &str) -> Result<Self> {
        serde_yaml::from_str(cfg).map_err(|x| anyhow!(x))
    }

    #[allow(dead_code)]
    pub fn to_cfg(&self) -> Result<String> {
        serde_yaml::to_string(self).map_err(|x| anyhow!(x))
    }
}

#[derive(Parser)]
pub struct Cli {
    /// Path of the configuration file
    #[arg(short, long, default_value_t = String::from("openapi-server/dist/develop.yml"))]
    pub cfg: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TlsConfig {
    pub cert: String,
    pub key: String,
}

impl TlsConfig {
    #[allow(dead_code)]
    pub fn get_tls_identity(&self) -> Identity {
        let cert = fs::read_to_string(self.cert.clone())
            .map_err(|x| anyhow!("Failed {} {}", self.cert, x.to_string()))
            .unwrap();
        let key = fs::read_to_string(self.key.clone())
            .map_err(|x| anyhow!("Failed {} {}", self.key, x.to_string()))
            .unwrap();
        Identity::from_pem(cert, key)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TelegramAuthConfig {
    pub telegram_api_id: i32,
    pub telegram_api_hash: String,
    pub client_id: String,
    pub token_auth_bot: String,
    pub next_public_server_login_author: String,
    pub next_public_server_login_with_telegram: String,
    pub next_public_torii: String,
    #[serde(rename = "session-path")]
    pub session_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cfg: &str = "
        grpc-listen: 0.0.0.0:50051
        openapi-listen: 0.0.0.0:50052
        rpc-client:
            zion-rpc: https://devnet-rpc.zionx.network
            torii-rpc: https://torii.zionx.network
            chain-id: 176923
        tls:
            cert: openapi-server/dist/tls/server.pem
            key: openapi-server/dist/tls/server.key
        auth-secret: my-secret
        doc-path: docs/openapi
        telegram-auth:
            telegram_api_id: 25038662
            telegram_api_hash: a50958c4a91203f1e443b9ea10df39a3
            client_id: \"7109740482\"
            token_auth_bot: \"7109740482:AAGhzij1EY8NNFo73d7fg9fJg0lAATclWYw\"
            next_public_server_login_author: \"https://teleauthy.zionx.network\"
            next_public_server_login_with_telegram: \"https://zklogin.zionx.network\"
            next_public_torii: \"https://torii.zionx.network/torii\"
            session-path: openapi-server/dist/sessions
        private-key-path: webhook-etherman/dist/private_key.pem
        db-url: postgresql://sagi:123@localhost:15432/sagi
        etherman:
            ethereum-rpc: https://devnet-rpc.zionx.network
            chain-id: 176923
        ";
        let c = Config::from_cfg(cfg).unwrap();
        Config::to_cfg(&c).unwrap();
    }
}
