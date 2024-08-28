import "@openzeppelin/hardhat-upgrades";
import { ethers, upgrades } from "hardhat";
import { AbiCoder, Contract } from "ethers";

async function main() {
    const [owner] = await ethers.getSigners();

    console.log("Deploying...");
    const ERC20 = await ethers.getContractFactory("TokenERC20");
    const erc20 = await ERC20.deploy(owner.address, "TestTokenERC20", "TTE20", ethers.parseEther("1000000000"));
    await erc20.waitForDeployment();

    console.log(await erc20.getAddress());
}

main();