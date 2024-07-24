use {
    anyhow::{anyhow, Result},
    clap::Parser,
    openapi_ethers::config::Config as EthereumClientConfig,
    serde::{Deserialize, Serialize},
    std::{fs, net::SocketAddr},
    tonic::transport::Identity,
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

    #[serde(rename = "auth-secret")]
    pub auth_secret: String,

    #[serde(rename = "doc-path")]
    pub doc_path: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cfg: &str = "
        grpc-listen: 0.0.0.0:50051        
        openapi-listen: 0.0.0.0:50052        
        rpc-client:
            ethereum-rpc: https://devnet-rpc.zionx.network
            chain-id: 176923
            deployer-keystore: openapi-server/dist/deployer-develop.keystore
        tls:
            cert: openapi-server/dist/tls/server.pem
            key: openapi-server/dist/tls/server.key
        auth-secret: my-secret
        doc-path: docs/openapi
        ";
        let c = Config::from_cfg(cfg).unwrap();
        Config::to_cfg(&c).unwrap();
    }
}
