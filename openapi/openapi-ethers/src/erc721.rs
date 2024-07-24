use ethers_contract::abigen;

abigen!(ERC721, "./dist/erc721/abi.json");

pub fn erc721_abi() -> String {
    include_str!("../dist/erc721/abi.json").to_string()
}

pub fn erc721_bytecode() -> Vec<u8> {
    include_bytes!("../dist/erc721/bytecode.bin").to_vec()
}
