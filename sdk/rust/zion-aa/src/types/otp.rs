use std::vec::Vec;

// Structs corresponding to TypeScript interfaces

#[derive(Debug, Clone)]
pub struct ZkOTPInput {
    pub time: u64, // Rust doesn't have a `number` type, assuming it is an integer
    pub otp: String,
    pub path_elements: Vec<u128>, // Use u128 for large integers
    pub path_index: Vec<u32>,     // Use u32 for smaller integers
}

#[derive(Debug, Clone)]
pub struct ZkProof {
    pub p_a: Vec<String>,
    pub p_b: Vec<Vec<String>>,
    pub p_c: Vec<String>,
    pub pub_signals: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ZkPath {
    pub path_wasm: String,
    pub path_zkey: String,
}

#[derive(Debug, Clone)]
pub struct RecoveryOTPOptions {
    pub path_wasm: String,
    pub path_zkey: String,
    pub layer: u32,
    pub hashes: Vec<u128>, // Use u128 for large integers
    pub code: Option<String>,
    pub time: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct OTPOptions {
    pub path_wasm: String,
    pub path_zkey: String,
    pub layer: u32,
    pub hashes: Vec<u128>, // Use u128 for large integers
    pub code: Option<String>,
    pub time: Option<u64>,
    pub secret: Option<String>,
    pub hashes_deadline: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ZkLeaf {
    pub left: u128,  // Use u128 for large integers
    pub right: u128, // Use u128 for large integers
}

#[derive(Debug, Clone)]
pub struct ZkRoot {
    pub root: u128,        // Use u128 for large integers
    pub hashes: Vec<u128>, // Use u128 for large integers
}

// Type alias corresponding to TypeScript type alias

pub type ZkLeafBuilder = fn(leaf: u64) -> ZkLeaf; // Assuming `leaf` is an integer
