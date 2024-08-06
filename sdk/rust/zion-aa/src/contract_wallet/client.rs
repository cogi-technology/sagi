use anyhow::{anyhow, Result};
use ethers::{
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer},
};
use ethers_core::k256::schnorr::SigningKey;
use ethers_providers::{Http, Provider, ProviderExt};
use rand::rngs::OsRng;
use std::sync::Arc;

pub type Client = SignerMiddleware<Arc<Provider<Http>>, LocalWallet>;

#[async_trait::async_trait]
pub trait ClientMethods {
    async fn try_new(rpc_endpoint: &str, chain_id: u64, signer_privatekey: &str) -> Result<Client>;
    async fn random_wallet(rpc_endpoint: &str, chain_id: u64) -> Result<Client>;
}

#[async_trait::async_trait]
impl ClientMethods for Client {
    #[inline]
    async fn try_new(rpc_endpoint: &str, chain_id: u64, signer_privatekey: &str) -> Result<Self> {
        let provider = Provider::<Http>::connect(rpc_endpoint).await;
        let signer = signer_privatekey
            .parse::<LocalWallet>()
            .map_err(|x| anyhow!("decrypt_keystore failed err:{}", x))?;
        let client = SignerMiddleware::new(Arc::new(provider), signer.with_chain_id(chain_id));

        Ok(client)
    }

    #[inline]
    async fn random_wallet(rpc_endpoint: &str, chain_id: u64) -> Result<Self> {
        let provider = Provider::<Http>::connect(rpc_endpoint).await;
        // Generate a random signing key
        let signing_key = SigningKey::random(&mut OsRng).to_bytes().to_vec();
        let signer = LocalWallet::from_bytes(&signing_key)?;
        let client = SignerMiddleware::new(Arc::new(provider), signer.with_chain_id(chain_id));
        Ok(client)
    }
}
