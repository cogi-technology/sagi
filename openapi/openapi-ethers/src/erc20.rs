use ethers_contract::abigen;

abigen!(ERC20, "./dist/erc20/abi.json");

pub fn erc20_abi() -> String {
    include_str!("../dist/erc20/abi.json").to_string()
}

pub fn erc20_bytecode() -> Vec<u8> {
    include_bytes!("../dist/erc20/bytecode.bin").to_vec()
}
