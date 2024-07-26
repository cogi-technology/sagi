use ethers::signers::Signer;
use ethers::types::Signature;
use std::sync::Arc;

pub async fn signer_sign<S>(
    hash: String,
    signer: Arc<S>,
) -> Result<Signature, Box<dyn std::error::Error>>
where
    S: Signer,
    S::Error: 'static,
{
    // Sign the message (in ethers-rs, the equivalent of `signMessage` is `sign_hash`)
    let signature = signer.sign_message(hash.into_bytes()).await?;

    Ok(signature)
}
