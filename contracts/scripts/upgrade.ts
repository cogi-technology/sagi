import "@openzeppelin/hardhat-upgrades";
import { ethers, upgrades } from 'hardhat';
import * as dotenv from 'dotenv';
dotenv.config();

async function main() {
    const LockNew = await ethers.getContractFactory("LockUpgradeableV2");
    console.log('Upgrading Lock...');
    let newLock = await upgrades.upgradeProxy(process.env.CONTRACT as string, LockNew);
    console.log('Lock upgraded');

    console.log("New version:", await newLock.version());
}

main();