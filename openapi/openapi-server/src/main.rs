mod config;
mod server;
mod services;
mod helpers;

use {
    anyhow::{anyhow, Result},
    clap::Parser,
    config::{Cli, Config},
    openapi_ethers::client,
    openapi_logger::{info, init as logger_init},
    server::{run as run_server, ServerConfig},
    std::{env, fs},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //logger
    logger_init(None::<String>, true)?;

    let args = Cli::parse();
    let cfg = fs::read_to_string(args.cfg.clone())
        .map_err(|x| anyhow!("Failed {} {}", args.cfg, x.to_string()))
        .unwrap();
    let c = Config::from_cfg(cfg.as_str())
        .map_err(|x| anyhow!("Failed {} {}", args.cfg, x.to_string()))
        .unwrap();

    dotenv::dotenv().ok();
    let key_password = env::var("KEY_PASSWORD")?;

    let rpc_client = client::init_client(c.rpc_client, key_password).await?;
    // let identity = c.tls.get_tls_identity();

    let server_config = ServerConfig {
        auth_secret: c.auth_secret,
        doc_path: c.doc_path,
        // tls_identity: identity,
        grpc_addr: c.grpc_listen,
        openapi_addr: c.openapi_listen,
    };

    info!("Started at {}", c.grpc_listen);
    tokio::select! {
        _ = async move {
            run_server(rpc_client, server_config).await
        } => {},
    }

    Ok(())
}
