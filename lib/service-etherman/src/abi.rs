use ethers_contract::abigen;

abigen!(
    KogiERC721,
    "./dist/KogiERC721.abi.json",
    derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    KogiERC20,
    "./dist/KogiERC20.abi.json",
    derives(serde::Deserialize, serde::Serialize)
);