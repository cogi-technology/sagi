import "@openzeppelin/hardhat-upgrades";
import { ethers, upgrades } from "hardhat";
import { AbiCoder, BigNumberish, Contract } from "ethers";

async function main() {
    const [owner, user1] = await ethers.getSigners();

    const erc404 = await ethers.getContractAt("TokenERC404", "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512");

    const tx = await erc404.transfer(user1.address, ethers.parseEther("1000"));
    const receipt = await tx.wait();

    const balance_erc20 = await erc404["balanceOf(address)"](user1.address);
    let balance_erc1155 = await erc404["balanceOf(address,uint256)"](user1.address, 3);

    console.log(balance_erc20);
    console.log(balance_erc1155);
}

main();