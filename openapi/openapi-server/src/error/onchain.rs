use {ethers::types::U256, hex::FromHex, std::str};

#[allow(dead_code)]
pub fn decode_revert_reason(data: String) -> String {
    if data.len() < 10 {
        return "Invalid revert reason data".to_string();
    }
    // Extract the selector
    let selector = &data[0..10];

    // Check if the selector is known
    match selector {
        // Add known selectors and their decoding logic
        "0x08c379a0" => {
            // This is a default error(string) selector
            let revert_data = &data[10..];
            if let Ok(bytes) = Vec::from_hex(revert_data) {
                if bytes.len() >= 64 {
                    let str_len = usize::from_be_bytes(bytes[32..64].try_into().unwrap());
                    if bytes.len() >= 64 + str_len {
                        if let Ok(reason) = str::from_utf8(&bytes[64..64 + str_len]) {
                            return reason.to_string();
                        }
                    }
                }
            }
        }
        "0xe450d38c" => {
            // Custom error selector, decode accordingly
            return decode_custom_error(data);
        }
        _ => {
            // Unknown selector
            return format!("Unknown error selector: {}", selector);
        }
    }
    "Unable to decode revert reason".to_string()
}

#[allow(dead_code)]
fn decode_custom_error(data: String) -> String {
    // Here you would define how to decode the specific custom error
    // For demonstration purposes, let's assume it's a custom error with specific fields

    if let Ok(bytes) = Vec::from_hex(&data[10..]) {
        // Example: address, uint256, and another uint256
        if bytes.len() >= 96 {
            let address = ethers::types::H160::from_slice(&bytes[0..20]);
            let first_uint = U256::from_big_endian(&bytes[20..52]);
            let second_uint = U256::from_big_endian(&bytes[52..84]);
            return format!(
                "Custom Error: address: {}, first uint: {}, second uint: {}",
                address, first_uint, second_uint
            );
        }
    }
    "Unable to decode custom error".to_string()
}
