use {
    super::config::Config,
    anyhow::{anyhow, Result},
    ethers::{
        middleware::SignerMiddleware,
        providers::Provider,
        signers::{LocalWallet, Signer},
    },
    ethers_providers::{Http, ProviderExt},
    std::sync::Arc,
};

pub type Client = SignerMiddleware<Arc<Provider<Http>>, LocalWallet>;

pub async fn init_client(config: Config, key_password: String) -> Result<Arc<Client>> {
    let Config {
        ethereum_rpc,
        chain_id,
        deployer_keystore,
    } = config;

    let provider = Arc::new(Provider::<Http>::connect(ethereum_rpc.as_str()).await);
    let deployer = LocalWallet::decrypt_keystore(deployer_keystore, key_password)
        .map_err(|x| anyhow!("decrypt_keystore failed err:{}", x))?;
    let client = Arc::new(SignerMiddleware::new(
        Arc::clone(&provider),
        deployer.with_chain_id(chain_id),
    ));

    Ok(client)
}
