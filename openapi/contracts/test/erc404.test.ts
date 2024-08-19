import { expect } from "chai";
import "@openzeppelin/hardhat-upgrades";
import { ethers } from "hardhat";
import {
    loadFixture,
} from "@nomicfoundation/hardhat-network-helpers";
import { BigNumberish } from "ethers";

export const toWei = (ether: number): BigNumberish => {
    return ethers.parseEther(ether.toString())
}

describe("TokenERC404", () => {
    async function deployTokenFixture() {
        const [owner] = await ethers.getSigners();

        const initialSupply = 10_000_000_000;
        const units = 4;
        const ids = [0, 1, 2, 3, 4, 5, 6];
        const Token = await ethers.getContractFactory("TokenERC404");
        const token = await Token.deploy(
            owner.address,
            "TestTokenERC404",
            "TTE404",
            6,
            toWei(initialSupply),
            toWei(units),
            ids,
            "https://baseuri.com/"
        );
        await token.waitForDeployment();
        const zeroAddress = "0x0000000000000000000000000000000000000000";
        return { token, initialSupply, units, ids, zeroAddress };
    }

    it("balanceOf", async () => {
        const { token, initialSupply, ids, zeroAddress } = await loadFixture(deployTokenFixture);
        const [owner] = await ethers.getSigners();
        expect(await token["balanceOf(address)"](owner.address)).to.equal(toWei(initialSupply));
        const res = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => owner), ids);

        let balance = BigInt(0);
        for (let i in res) {
            balance += BigInt(res[i]);
        }
        expect(balance).to.equal(0);
    });

    it("transfer isFromExempt && isToExempt", async function () {
        const { token, ids } = await loadFixture(deployTokenFixture);
        const [_, user1] = await ethers.getSigners();
        await token.addTransferExempt(user1);
        const value1 = 1_000;
        await token.transfer(user1, toWei(value1));
        expect(await token["balanceOf(address)"](user1)).to.equal(toWei(value1));
        const res = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user1), ids);
        let balance = BigInt(0);
        for (let i in res) {
            balance += BigInt(res[i]);
        }
        expect(balance).to.equal(0);
    });

    it("transfer isSendOnly", async function () {
        const { units, token, ids } = await loadFixture(deployTokenFixture);
        const [owner, user1, user2] = await ethers.getSigners();

        //setup
        await token.addTransferExempt(user2);
        const value1 = 1_000;
        await token.transfer(user1, toWei(value1));
        expect(await token["balanceOf(address)"](user1)).to.equal(toWei(value1));
        let res1 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user1), ids);
        let balance1 = BigInt(0);
        for (let i in res1) {
            balance1 += BigInt(res1[i]);
        }
        expect(balance1).to.equal(value1 / units);

        //transfer user1 -> user2
        const value2 = 500;
        const token1 = token.connect(user1);
        await token1.transfer(user2, toWei(value2));

        //check user1
        const b1 = await token["balanceOf(address)"](user1.address);
        expect(b1).to.equal(toWei(value1 - value2));
        res1 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user1), ids);
        balance1 = BigInt(0);
        for (let i in res1) {
            balance1 += BigInt(res1[i]);
        }
        expect(balance1).to.equal(BigInt(value1 - value2) / BigInt(units));

        //check user2
        expect(await token["balanceOf(address)"](user2.address)).to.equal(toWei(value2));
        const res2 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user2), ids);
        let balance2 = BigInt(0);
        for (let i in res2) {
            balance2 += BigInt(res2[i]);
        }
        expect(balance2).to.equal(0);
    });

    it("transfer isReceiveOnly", async function () {
        const { units, token, ids } = await loadFixture(deployTokenFixture);
        const [owner, user1, user2] = await ethers.getSigners();

        //setup
        await token.addTransferExempt(user1);
        const value1 = 1_000;
        await token.transfer(user1, toWei(value1));
        expect(await token["balanceOf(address)"](user1.address)).to.equal(toWei(value1));
        let res1 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user1), ids);
        let balance1 = BigInt(0);
        for (let i in res1) {
            balance1 += BigInt(res1[i]);
        }
        expect(balance1).to.equal(0);

        //transfer user1 -> user2
        const value2 = 500;
        const token1 = token.connect(user1);
        await token1.transfer(user2, toWei(value2));

        //check user1
        const b1 = await token["balanceOf(address)"](user1.address);
        expect(b1).to.equal(toWei(value1 - value2));
        res1 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user1), ids);
        balance1 = BigInt(0);
        for (let i in res1) {
            balance1 += BigInt(res1[i]);
        }
        expect(balance1).to.equal(0);

        //check user2
        expect(await token["balanceOf(address)"](user2.address)).to.equal(toWei(value2));
        const res2 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user2), ids);
        let balance2 = BigInt(0);
        for (let i in res2) {
            balance2 += BigInt(res2[i]);
        }
        expect(balance2).to.equal(value2 / units);
    });

    it("transfer isTransfer", async function () {
        const { units, token, ids } = await loadFixture(deployTokenFixture);
        const [owner, user1, user2] = await ethers.getSigners();

        //setup    
        const value1 = 1_000;
        await token.transfer(user1, toWei(value1));
        expect(await token["balanceOf(address)"](user1.address)).to.equal(toWei(value1));
        let res1 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user1), ids);
        let balance1 = BigInt(0);
        for (let i in res1) {
            balance1 += BigInt(res1[i]);
        }
        expect(balance1).to.equal(value1 / units);

        //transfer user1 -> user2
        const value2 = 400;
        const token1 = token.connect(user1);
        await token1.transfer(user2, toWei(value2));

        //check user1
        const b1 = await token["balanceOf(address)"](user1.address);
        expect(b1).to.equal(toWei(value1 - value2));
        res1 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user1), ids);
        balance1 = BigInt(0);
        for (let i in res1) {
            balance1 += BigInt(res1[i]);
        }
        expect(balance1).to.equal((value1 - value2) / units);

        //check user2
        expect(await token["balanceOf(address)"](user2.address)).to.equal(toWei(value2));
        const res2 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user2), ids);
        let balance2 = BigInt(0);
        for (let i in res2) {
            balance2 += BigInt(res2[i]);
        }
        expect(balance2).to.equal(value2 / units);
    });

    it("transfer isTransfer + mint receiver", async function () {
        const { units, token, ids } = await loadFixture(deployTokenFixture);
        const [owner, user1, user2] = await ethers.getSigners();

        //setup user 1   
        const value1 = 4 * 100 + 3;
        await token.transfer(user1, toWei(value1));
        expect(await token["balanceOf(address)"](user1.address)).to.equal(toWei(value1));
        let res1 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user1), ids);
        let balance1 = BigInt(0);
        for (let i in res1) {
            balance1 += BigInt(res1[i]);
        }

        expect(balance1).to.equal(Math.floor(value1 / units));

        //setup user 2
        const value2 = 1
        await token.transfer(user2, toWei(value2));
        expect(await token["balanceOf(address)"](user2.address)).to.equal(toWei(value2));
        let res2 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user2), ids);
        let balance2 = BigInt(0);
        for (let i in res2) {
            balance2 += BigInt(res2[i]);
        }

        expect(balance2).to.equal(Math.floor(value2 / units));

        //transfer user1 -> user2
        const t2 = 4 * 3 + 3;
        const token1 = token.connect(user1);
        await token1.transfer(user2, toWei(t2));

        //check user1
        const b1 = await token["balanceOf(address)"](user1.address);

        expect(b1).to.equal(toWei(value1 - t2));
        res1 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user1), ids);
        balance1 = BigInt(0);
        for (let i in res1) {
            balance1 += BigInt(res1[i]);
        }
        expect(balance1).to.equal(Math.floor((value1 - t2) / units));

        //check user2
        expect(await token["balanceOf(address)"](user2.address)).to.equal(toWei(value2 + t2));

        res2 = await token.erc1155BalanceOfBatch(Array.from({ length: ids.length }, (_, __) => user2), ids);
        balance2 = BigInt(0);
        for (let i in res2) {
            balance2 += BigInt(res2[i]);
        }
        expect(balance2).to.equal(Math.floor((value2 + t2) / units));
    });
});