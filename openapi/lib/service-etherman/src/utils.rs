use std::{collections::HashMap, fs::File, io::Read};

use crate::error::{into_anyhow, Result as ServiceResult};
use openssl::{
    pkey::{PKey, Private, Public},
    rsa::{Padding, Rsa},
    sign::{Signer, Verifier},
};
use reqwest::{Client, Method, RequestBuilder};
use serde::Serialize;
use tonic::Response;

#[macro_export]
macro_rules! tokio_sleep_ms {
    ($n: expr) => {{
        tokio::time::sleep(std::time::Duration::from_millis($n)).await;
    }};
}

#[macro_export]
macro_rules! db_string {
    ($n: expr) => {{
        format!("{:#x}", $n)
    }};
}

pub fn split_range(start: u64, end: u64, step: usize) -> impl Iterator<Item = (u64, u64)> {
    (start..end)
        .step_by(step)
        .map(move |current| (current, std::cmp::min(end, current + step as u64)))
}

pub fn get_signature(
    data: &str,
    private_key: PKey<Private>
) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    // Generate a 2048-bit RSA key pair
    // let rsa = Rsa::generate(2048)?;
    // let private_key = PKey::from_rsa(rsa.clone())?; // Clone to reuse the key
    // let public_key = PKey::from_rsa(rsa)?;

    // Sign the data using the private key
    let mut signer = Signer::new(openssl::hash::MessageDigest::sha512(), &private_key)?;
    signer.set_rsa_padding(Padding::PKCS1)?;
    signer.update(data.as_bytes())?;
    let signature = signer.sign_to_vec()?;
    println!("Signature: {:?}", signature);
    Ok(signature)
}

pub fn verify_signature(
    signature: Vec<u8>,
    data: &str,
    public_key: PKey<Public>,
) -> Result<bool, openssl::error::ErrorStack> {
    // Verify the signature using the public key
    let mut verifier = Verifier::new(openssl::hash::MessageDigest::sha512(), &public_key)?;
    verifier.set_rsa_padding(Padding::PKCS1)?;
    verifier.update(data.as_bytes())?;
    let is_valid = verifier.verify(&signature)?;

    Ok(is_valid)
}

pub fn load_private_key_from_file(filename: &str) -> Result<PKey<Private>, Box<dyn std::error::Error>> {
    // Read the PEM file contents
    let mut file = File::open(filename)?;
    let mut pem_data = Vec::new();
    file.read_to_end(&mut pem_data)?;

    // Load the RSA private key from the PEM data
    let rsa = Rsa::private_key_from_pem(&pem_data)?;
    let private_key = PKey::from_rsa(rsa)?;
    Ok(private_key)
}

pub fn load_public_key_from_file(filename: &str) -> Result<PKey<Public>, Box<dyn std::error::Error>> {
    // Read the PEM file contents
    let mut file = File::open(filename)?;
    let mut pem_data = Vec::new();
    file.read_to_end(&mut pem_data)?;

    // Load the RSA public key from the PEM data
    let rsa = Rsa::public_key_from_pem(&pem_data)?;
    let public_key = PKey::from_rsa(rsa)?;
    Ok(public_key)
}

pub async fn send_request_text<U>(
    client: &Client,
    method: Method,
    url: &str,
    body: Option<&U>,
    headers: Option<HashMap<String, String>>,
) -> ServiceResult<Response<String>>
where
    U: Serialize,
{
    let mut request: RequestBuilder = client.request(method, url);

    // Set the body if provided
    if let Some(b) = body {
        request = request.json(b);
    }

    // Set headers if provided
    if let Some(h) = headers {
        for (key, value) in h {
            request = request.header(&key, &value);
        }
    }

    let response = match request.send().await {
        Ok(response) => response,
        Err(e) => return Err(into_anyhow(e.into())),
    };

    match response.text().await {
        Ok(response) => Ok(Response::new(response)),
        Err(e) => Err(into_anyhow(e.into())),
    }
}
