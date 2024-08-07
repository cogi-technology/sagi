mod cache;
mod config;
mod entity;
mod error;
mod helpers;
mod server;
mod services;

use {
    anyhow::{anyhow, Result},
    cache::remove_expired_jwt_cache,
    clap::Parser,
    config::{Cli, Config},
    openapi_ethers::provider,
    openapi_logger::{info, init as logger_init},
    server::{run as run_server, ServerConfig},
    std::fs,
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

    let (zion_provider, torii_provider) = provider::init_provider(c.rpc_client).await?;
    // let identity = c.tls.get_tls_identity();

    // sessions user
    let dir_sessions = c.telegram_auth.session_path.as_str();
    if let Ok(_) = std::fs::remove_dir_all(dir_sessions) {
        std::fs::create_dir_all(dir_sessions).unwrap();
    } else {
        std::fs::create_dir_all(dir_sessions).unwrap();
    }
    //
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
            run_server(zion_provider, torii_provider, server_config, c.telegram_auth).await
        } => {},
        _ = async move {
            remove_expired_jwt_cache().await
        } => {},
    }

    Ok(())
}
