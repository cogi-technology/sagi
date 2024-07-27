use ethers_contract::abigen;

abigen!(EntryPoint, "./src/contracts/abi/EntryPoint.json");
abigen!(Account, "./src/contracts/abi/Account.json");
abigen!(Factory, "./src/contracts/abi/Factory.json");
