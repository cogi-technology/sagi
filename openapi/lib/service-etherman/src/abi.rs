use ethers_contract::abigen;

abigen!(RewardPool, "./dist/reward-pool-abi.json", derives(serde::Deserialize, serde::Serialize););
abigen!(KogiERC721, "./dist/KogiERC721.abi.json", derives(serde::Deserialize, serde::Serialize););
