use ethers::{types::Bytes, utils::hex};
use ethers_contract::abigen;

abigen!(ERC20, "./dist/erc20/abi.json");

pub fn erc20_bytecode() -> Bytes {
    hex::decode(include_str!("../dist/erc20/bytecode.bin"))
        .unwrap()
        .into()
}
