import "@openzeppelin/hardhat-upgrades";
import { ethers, upgrades } from "hardhat";
import { AbiCoder, BigNumberish, Contract } from "ethers";

export const toWei = (ether: number): BigNumberish => {
    return ethers.parseEther(ether.toString())
}

async function main() {
    const [owner] = await ethers.getSigners();

    console.log("Deploying...");
    const initialSupply = 10_000_000_000;
    const units = 4;
    const ids = [0, 1, 2, 3, 4, 5, 6];
    const Token = await ethers.getContractFactory("TokenERC404");
    const token = await Token.deploy(
        owner.address,
        "TestTokenERC404",
        "TTE404",
        toWei(initialSupply),
        toWei(units),
        ids,
        "https://baseuri.com/"
    );
    await token.waitForDeployment();

    console.log(await token.getAddress());
}

main();