// mod zero_knowledge;
mod jwt;

use crate::types::{
    jwt::ProofPoints,
    key::RoleWeight,
    user_operation::{request::UserOperationRequest, UserOperationSigned},
};
use anyhow::{anyhow, Result};
use ethers::{
    abi::{encode, Token},
    signers::LocalWallet,
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockNumber, Bytes,
        Eip1559TransactionRequest, Eip2930TransactionRequest, TransactionRequest, U256,
    },
    utils::{keccak256, rlp},
};
use ethers_providers::Middleware;
use std::fmt::Write;
use std::sync::Arc;

pub use jwt::decode_jwt;

#[macro_export]
macro_rules! tokio_sleep_ms {
    ($n: expr) => {{
        tokio::time::sleep(std::time::Duration::from_millis($n)).await;
    }};
}

#[macro_export]
macro_rules! address_to_string {
    ($n: expr) => {{
        format!("{:#x}", $n)
    }};
}

pub fn serialize_role_weight(role_weight: &RoleWeight) -> usize {
    (role_weight.owner_weight as usize) << 16
        | ((role_weight.assets_op_weight as usize) << 8)
        | (role_weight.guardian_weight as usize)
}

/// If possible, parses address from the first 20 bytes
pub fn get_address(buf: &[u8]) -> Option<Address> {
    if buf.len() >= 20 {
        Some(Address::from_slice(&buf[0..20]))
    } else {
        None
    }
}

fn bytes_to_typed_transaction(tx_bytes: &Bytes) -> Result<TypedTransaction> {
    // Try decoding the bytes using RLP
    let rlp_stream = rlp::Rlp::new(tx_bytes);

    // Detect the transaction type and decode accordingly
    if tx_bytes[0] <= 0x7f {
        // Legacy transaction
        let tx: TransactionRequest = rlp_stream.as_val()?;
        Ok(TypedTransaction::Legacy(tx))
    } else if tx_bytes[0] == 0x01 {
        // EIP-2930 transaction
        let tx: Eip2930TransactionRequest = rlp_stream.as_val()?;
        Ok(TypedTransaction::Eip2930(tx))
    } else if tx_bytes[0] == 0x02 {
        // EIP-1559 transaction
        let tx: Eip1559TransactionRequest = rlp_stream.as_val()?;
        Ok(TypedTransaction::Eip1559(tx))
    } else {
        Err(anyhow!("Unknown transaction type"))
    }
}

fn call_data_cost(data: Bytes) -> U256 {
    let cost: usize = data.iter().map(|&x| if x == 0 { 4 } else { 16 }).sum();

    U256::from(cost)
}

pub async fn fill_user_op<M: Middleware + 'static>(
    op: UserOperationRequest,
    provider: Arc<M>,
) -> Result<UserOperationSigned> {
    let mut op1 = op;
    // let account = Account::new(op1.sender, Arc::clone(&provider));
    // op1.nonce = account.nonce().await?;

    if op1.call_gas_limit.is_none() {
        let gas_estimated = provider
            .estimate_gas(
                &bytes_to_typed_transaction(&op1.call_data)?,
                Some(BlockNumber::Latest.into()),
            )
            .await?;
        op1.call_gas_limit = Some(gas_estimated);
    }

    let default_user_operation = UserOperationSigned::default().into_inner();

    if op1.max_fee_per_gas.is_none() {
        let block = provider
            .get_block(BlockNumber::Latest)
            .await?
            .ok_or_else(|| anyhow!("Latest BlockNumber is None"))?;

        let base_fee_per_gas = block.base_fee_per_gas.unwrap_or_default();

        op1.max_fee_per_gas = Some(
            base_fee_per_gas
                + op1
                    .max_priority_fee_per_gas
                    .unwrap_or_else(|| default_user_operation.max_priority_fee_per_gas),
        );
    }

    if op1.max_priority_fee_per_gas.is_none() {
        op1.max_priority_fee_per_gas = Some(default_user_operation.max_priority_fee_per_gas);
    }

    if op1.verification_gas_limit.is_none() {
        op1.verification_gas_limit = Some(default_user_operation.verification_gas_limit);
    }
    if op1.signature.is_none() {
        op1.signature = Some(default_user_operation.signature);
    }

    if op1.pre_verification_gas.is_none() {
        op1.pre_verification_gas = Some(default_user_operation.pre_verification_gas);
    }

    let mut ret = UserOperationSigned::from(op1);

    if ret.into_inner().pre_verification_gas.is_zero() {
        ret.mut_inner().pre_verification_gas = call_data_cost(ret.pack_without_signature());
    }

    Ok(ret)
}

pub fn get_provider_hashed(iss: String, aud: String) -> [u8; 32] {
    let iss_in_hex = iss.into_bytes();
    let aud_in_hex = aud.into_bytes();

    keccak256(encode(&[
        Token::Bytes(iss_in_hex),
        Token::Bytes(aud_in_hex),
    ]))
}

// Function to convert U256 to 256-bit hex string with padding
fn p256(n: U256) -> String {
    let mut nstr = format!("{:x}", n);
    while nstr.len() < 64 {
        nstr = format!("0{}", nstr);
    }
    format!("\"0x{}\"", nstr)
}

// Function to unstringify big integers from strings
fn unstringify_bigints(arr: &[String]) -> Vec<U256> {
    arr.iter().map(|s| U256::from_dec_str(s).unwrap()).collect()
}

// Function to convert proof and public inputs to Solidity call data
pub async fn groth16_export_solidity_call_data(
    proof: ProofPoints,
    pub_inputs: Vec<String>,
) -> String {
    let proof_pi_a = unstringify_bigints(&proof.pi_a);
    let proof_pi_b: Vec<Vec<U256>> = proof
        .pi_b
        .iter()
        .map(|vec| unstringify_bigints(vec))
        .collect();
    let proof_pi_c = unstringify_bigints(&proof.pi_c);
    let pub_inputs = unstringify_bigints(&pub_inputs);

    let mut inputs = String::new();
    for (i, pub_val) in pub_inputs.iter().enumerate() {
        if i != 0 {
            write!(inputs, ",").unwrap();
        }
        write!(inputs, "{}", p256(*pub_val)).unwrap();
    }

    let s = format!(
        "[{}, {}],[[{}, {}],[{}, {}]],[{}, {}],[{}]",
        p256(proof_pi_a[0]),
        p256(proof_pi_a[1]),
        p256(proof_pi_b[0][1]),
        p256(proof_pi_b[0][0]),
        p256(proof_pi_b[1][1]),
        p256(proof_pi_b[1][0]),
        p256(proof_pi_c[0]),
        p256(proof_pi_c[1]),
        inputs
    );

    s
}

pub fn make_pin_code_holder(code: String, salt: String) -> Result<LocalWallet> {
    let salt = hex::decode(salt)?.to_vec();

    let prv = hex::encode(keccak256(ethers::abi::encode_packed(&[
        Token::Bytes(code.into_bytes()),
        Token::FixedBytes(salt),
    ])?));

    prv.parse::<LocalWallet>().map_err(|e| e.into())
}
