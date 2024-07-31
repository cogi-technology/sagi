// Enum corresponding to TypeScript enum

#[derive(Debug, Clone, Copy)]
pub enum KeyType {
    Secp256k1,
    ERC1271Wallet,
    OpenIDWithEmail,
    OTP,
    RecoveryOTP,
    JWTZKProof,
    PINCode,
    None,
}

// Struct corresponding to TypeScript interface

#[derive(Debug, Clone)]
pub struct RoleWeight {
    pub owner_weight: u8, // Rust doesn't have a `number` type, assuming it is an integer
    pub assets_op_weight: u8, // Use u32 for smaller integers
    pub guardian_weight: u8, // Use u32 for smaller integers
}
