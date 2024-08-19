use {
    super::config::Config,
    anyhow::Result,
    ethers::providers::Provider,
    ethers_providers::{Http, ProviderExt},
    std::sync::Arc,
};

pub async fn init_provider(config: Config) -> Result<(Arc<Provider<Http>>, Arc<Provider<Http>>)> {
    let Config {
        zion_rpc,
        torii_rpc,
        chain_id: _,
    } = config;
    let zion_provider = Arc::new(Provider::<Http>::connect(zion_rpc.as_str()).await);
    let torii_provider = Arc::new(Provider::<Http>::connect(torii_rpc.as_str()).await);

    Ok((zion_provider, torii_provider))
}
