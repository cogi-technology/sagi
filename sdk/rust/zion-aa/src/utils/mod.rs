use std::sync::Arc;

use crate::{
    contracts::{Account, EntryPoint},
    types::{
        key::RoleWeight,
        user_operation::{request::UserOperationRequest, UserOperationSigned},
    },
};
use anyhow::{anyhow, Result};
use ethers::{
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockNumber, Bytes,
        Eip1559TransactionRequest, Eip2930TransactionRequest, TransactionRequest, U256,
    },
    utils::rlp,
};
use ethers_providers::Middleware;

mod zero_knowledge;

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
    entry_point: &EntryPoint<M>,
) -> Result<UserOperationSigned> {
    let mut op1 = op;
    let provider = Arc::clone(&entry_point.client());

    let account = Account::new(op1.sender, Arc::clone(&provider));

    op1.nonce = account.nonce().await?;

    if op1.call_gas_limit.is_none() {
        let gas_estimated = provider
            .estimate_gas(
                &bytes_to_typed_transaction(&op1.call_data)?,
                Some(BlockNumber::Latest.into()),
            )
            .await?;
        op1.call_gas_limit = Some(gas_estimated);
    }

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
                    .unwrap_or_else(|| UserOperationSigned::default().max_priority_fee_per_gas),
        );
    }

    if op1.max_priority_fee_per_gas.is_none() {
        op1.max_priority_fee_per_gas =
            Some(UserOperationSigned::default().max_priority_fee_per_gas);
    }

    if op1.verification_gas_limit.is_none() {
        op1.verification_gas_limit = Some(UserOperationSigned::default().verification_gas_limit);
    }
    if op1.signature.is_none() {
        op1.signature = Some(UserOperationSigned::default().signature);
    }

    if op1.pre_verification_gas.is_none() {
        op1.pre_verification_gas = Some(UserOperationSigned::default().pre_verification_gas);
    }

    let mut ret = UserOperationSigned::from(op1);

    if ret.pre_verification_gas.is_zero() {
        ret.pre_verification_gas = call_data_cost(ret.pack_without_signature());
    }

    Ok(ret)
}
