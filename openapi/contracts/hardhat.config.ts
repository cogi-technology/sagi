import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "@openzeppelin/hardhat-upgrades";
import "@nomicfoundation/hardhat-verify";
import "hardhat-abi-exporter";
import "dotenv/config";

const accounts = [
  process.env.DEPLOYER_PRV as string,
]

const local_accounts = [
  process.env.LOCAL_DEPLOYER_PRV as string,
  process.env.LOCAL_USER_PRV as string,
]

const config: HardhatUserConfig = {
  solidity: "0.8.24",
  networks: {
    hardhat: {},
    ganache: {
      url: "http://127.0.0.1:8545/",
      accounts: local_accounts,
    },
    // zionx: {
    //   url: "https://devnet-rpc.zionx.network",
    //   chainId: 176923,
    //   accounts
    // },
  },
  sourcify: {
    enabled: true,
  },
  etherscan: {
    apiKey: {
      ganache: `${process.env.ETHERSCAN_API_KEY as string}`
    },
    customChains: [
      {
        network: 'ganache',
        chainId: 1337,
        urls: {
          apiURL: 'http://localhost/api',
          browserURL: 'http://localhost',
        },
      },
    ]
  },
  abiExporter: {
    path: "./abi",
    clear: false,
    flat: true,
    pretty: true,
  },
};

export default config;
