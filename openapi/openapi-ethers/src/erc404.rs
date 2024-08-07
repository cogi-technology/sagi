use ethers::{types::Bytes, utils::hex};
use ethers_contract::abigen;

abigen!(ERC404, "./dist/erc404/abi.json");

pub fn erc404_bytecode() -> Bytes{
    hex::decode(include_str!("../dist/erc404/bytecode.bin"))
        .unwrap()
        .into()
}
