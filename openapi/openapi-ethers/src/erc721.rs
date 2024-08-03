use ethers::{types::Bytes, utils::hex};
use ethers_contract::abigen;

abigen!(ERC721, "./dist/erc721/abi.json");

pub fn erc721_bytecode() -> Bytes {
    hex::decode(include_str!("../dist/erc721/bytecode.bin"))
        .unwrap()
        .into()
}
