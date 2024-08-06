import "@openzeppelin/hardhat-upgrades";
import { ethers, upgrades } from "hardhat";
import { AbiCoder, Contract } from "ethers";

async function main() {
    const JAN_1ST_2030 = 1893456000;

    const Lock = await ethers.getContractFactory("LockUpgradeable");
    console.log("Deploying Lock...");
    const lock = await upgrades.deployProxy(Lock, [JAN_1ST_2030]);
    await lock.waitForDeployment();

    console.log("Lock deployed to:", await lock.getAddress());
    console.log("version:", await lock.version());
}

main();