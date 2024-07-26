use crate::types::key::RoleWeight;

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
