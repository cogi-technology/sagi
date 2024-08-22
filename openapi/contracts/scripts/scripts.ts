import "@openzeppelin/hardhat-upgrades";
import { ethers, upgrades } from "hardhat";
import { AbiCoder, BigNumberish, Contract } from "ethers";

async function main() {
    const [owner, user1] = await ethers.getSigners();

    // const erc404 = await ethers.getContractAt("TokenERC404", "0x91dd700e71087bd03de86b8e636528e93ed5cd4b");

    // const tx = await erc404.approve("0x31158C661D5a1266c7A7324EE9beBc84293a67B1", BigInt("100000000"));
    // const receipt = await tx.wait();

    // const balance_erc20 = await erc404["balanceOf(address)"](user1.address);
    // let balance_erc1155 = await erc404["balanceOf(address,uint256)"](user1.address, 3);

    // console.log(balance_erc20);
    // console.log(balance_erc1155);

    const erc20 = await ethers.getContractAt("TokenERC20", "0xfda900d99085a53dbbb23d1fe007f4bbb54f282d");
    const tx = await erc20.approve("0x31158C661D5a1266c7A7324EE9beBc84293a67B1", BigInt("100000000"));
    const receipt = await tx.wait();

    console.log(receipt?.hash);
}

main();