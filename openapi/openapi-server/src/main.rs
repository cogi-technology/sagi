mod cache;
mod config;
mod entity;
mod error;
mod helpers;
mod server;
mod services;

use {
    anyhow::{anyhow, Result},
    cache::{remove_expired_jwt_cache, remove_expired_session_cache},
    clap::Parser,
    config::{Cli, Config},
    openapi_ethers::provider,
    openapi_logger::{info, init as logger_init},
    server::{run as run_server, ServerConfig},
    std::{fs, sync::Arc},
    zion_service_db::database::Database,
    zion_service_etherman::{
        nft::{etherman::Etherman, nftwebhood::NFTWebhood},
        token::{etherman_token::EthermanToken, tokenwebhood::TokenWebhood},
    },
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
    let telegram_auth = c.telegram_auth.clone();
    let dir_sessions = telegram_auth.session_path.as_str();
    let _ = std::fs::remove_dir_all(dir_sessions).is_ok();
    std::fs::create_dir_all(dir_sessions).unwrap();

    // NFT
    // etherman
    let db: Arc<Database> = Arc::new(Database::new(c.db_url));
    let etherman_nft: Arc<Etherman> =
        Arc::new(Etherman::init(db.clone(), "test".into(), c.etherman.clone()).await?);
    // webhoood
    let webhood_nft: Arc<NFTWebhood> =
        Arc::new(NFTWebhood::init(db.clone(), c.private_key_path.clone()).await?);
    //
    // Token
    let etherman_token: Arc<EthermanToken> =
        Arc::new(EthermanToken::init(db.clone(), "test".into(), c.etherman.clone()).await?);
    // webhoood
    let webhood_token: Arc<TokenWebhood> =
        Arc::new(TokenWebhood::init(db.clone(), c.private_key_path.clone()).await?);
    //

    let server_config = ServerConfig {
        auth_secret: c.auth_secret,
        doc_path: c.doc_path,
        // tls_identity: identity,
        grpc_addr: c.grpc_listen,
        openapi_addr: c.openapi_listen,
        private_key_path: c.private_key_path,
    };
    info!("Started at {}", c.grpc_listen);
    tokio::select! {
        _ = webhood_token.heartbeat() => {},
        _ = etherman_token.heartbeat() => {},
        _ = webhood_nft.heartbeat() => {},
        _ = etherman_nft.heartbeat() => {},
        _ = async move {
            run_server(zion_provider, torii_provider, server_config, c.telegram_auth.clone(), db.clone()).await
        } => {},
        _ = async move {
            remove_expired_jwt_cache().await
        } => {},
        _ = async move {
            remove_expired_session_cache(dir_sessions).await
        } => {},
    }

    Ok(())
}
