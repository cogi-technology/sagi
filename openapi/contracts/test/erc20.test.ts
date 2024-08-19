import { expect } from "chai";
import "@openzeppelin/hardhat-upgrades";
import { ethers } from "hardhat";
import {
    loadFixture,
} from "@nomicfoundation/hardhat-network-helpers";

describe("ERC20", () => {
    async function deployTokenFixture() {
        const [owner, user1, user2] = await ethers.getSigners();

        const ERC20 = await ethers.getContractFactory("TokenERC20");
        const erc20 = await ERC20.deploy(owner.address, "TestTokenERC20", "TTE20", ethers.parseEther("1000000000"));
        await erc20.waitForDeployment();

        return { erc20, owner, user1, user2 };
    }

    describe("Happy paths", () => {
        it("token name and symbol set correctly", async function () {
            const { erc20 } = await loadFixture(deployTokenFixture);

            expect(await erc20.name()).to.equal("TestTokenERC20");
            expect(await erc20.symbol()).to.equal("TTE20");
        });

        it("totalsupply correct", async () => {
            const { erc20 } = await loadFixture(deployTokenFixture);

            const totalSupply = await erc20.totalSupply();
            expect(totalSupply).to.equal(ethers.parseEther("1000000000"));
        });

        it("transfer function moves tokens correctly", async function () {
            const { erc20, owner, user1 } = await loadFixture(deployTokenFixture);

            // Get initial balances
            const initialSenderBalance = await erc20.balanceOf(owner.address);
            const initialReceiverBalance = await erc20.balanceOf(user1.address);

            // Perform transfer
            await erc20.transfer(user1, 500);

            // Get final balances after transfer
            const finalSenderBalance = await erc20.balanceOf(owner.address);
            const finalReceiverBalance = await erc20.balanceOf(user1.address);

            // Check if balances are updated correctly
            expect(finalSenderBalance).to.equal(initialSenderBalance - BigInt(500));
            expect(finalReceiverBalance).to.equal(initialReceiverBalance + BigInt(500));
        });

        it("transferFrom function moves tokens correctly", async function () {
            const { erc20, owner, user1, user2 } = await loadFixture(deployTokenFixture);

            await erc20.transfer(user1.address, 1000);

            // Approve spender to spend tokens on behalf of owner
            await erc20.connect(user1).approve(owner.address, 500);

            // Transfer tokens from owner to receiver using spender's allowance
            await erc20.connect(owner).transferFrom(user1.address, user2.address, 300);

            // Check balance of owner and receiver after transfer
            expect(await erc20.balanceOf(user1.address)).to.equal(700);
            expect(await erc20.balanceOf(user2.address)).to.equal(300);

            // Check allowance of spender after transfer
            expect(await erc20.allowance(user1.address, owner.address)).to.equal(200);
        });
    });
});