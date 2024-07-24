use ethers_contract::abigen;

abigen!(ERC404, "./dist/erc404/abi.json");

pub fn erc404_abi() -> String {
    include_str!("../dist/erc404/abi.json").to_string()
}

pub fn erc404_bytecode() -> Vec<u8> {
    include_bytes!("../dist/erc404/bytecode.bin").to_vec()
}
