import { expect } from "chai";
import "@openzeppelin/hardhat-upgrades";
import { ethers } from "hardhat";
import {
    loadFixture,
} from "@nomicfoundation/hardhat-network-helpers";


describe("TokenERC721", function () {
    async function deployTokenFixture() {
        const Token = await ethers.getContractFactory("TokenERC721");
        const [owner, addr1, addr2] = await ethers.getSigners();
        const token = await Token.deploy(owner.address, "MyToken", "MTK", "https://baseuri.com/");
        await token.waitForDeployment();

        return { token, owner, addr1, addr2 };
    }

    describe("Deployment", function () {
        it("Should set the right owner", async function () {
            const { token, owner } = await loadFixture(deployTokenFixture);
            expect(await token.owner()).to.equal(owner.address);
        });

        it("Should set the right name and symbol", async function () {
            const { token } = await loadFixture(deployTokenFixture);
            expect(await token.name()).to.equal("MyToken");
            expect(await token.symbol()).to.equal("MTK");
        });
    });

    describe("Minting", function () {
        it("Should mint a new token and assign it to the right owner", async function () {
            const { token, owner, addr1 } = await loadFixture(deployTokenFixture);

            await token.mint(addr1.address);
            expect(await token.ownerOf(0)).to.equal(addr1.address);
        });

        it("Should increment tokenId correctly on multiple mints", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.mint(addr1.address);
            await token.mint(addr2.address);
            expect(await token.ownerOf(0)).to.equal(addr1.address);
            expect(await token.ownerOf(1)).to.equal(addr2.address);
        });

        it("Should fail if minting is attempted by non-owner", async function () {
            const { token, addr1 } = await loadFixture(deployTokenFixture);

            await expect(token.connect(addr1).mint(addr1.address)).to.be.revertedWithCustomError(token, "OwnableUnauthorizedAccount");;
        });
    });

    describe("Transfers", function () {
        it("Should transfer token between accounts", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.mint(addr1.address);
            await token.connect(addr1).transferFrom(addr1.address, addr2.address, 0);
            expect(await token.ownerOf(0)).to.equal(addr2.address);
        });

        it("Should fail if transfer is attempted by non-owner", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.mint(addr1.address);
            await expect(token.connect(addr2).transferFrom(addr1.address, addr2.address, 0)).to.be.revertedWithCustomError(token, "ERC721InsufficientApproval");
        });
    });

    describe("Approvals", function () {
        it("Should approve and then transfer a token by an approved account", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.mint(addr1.address);
            await token.connect(addr1).approve(addr2.address, 0);
            await token.connect(addr2).transferFrom(addr1.address, addr2.address, 0);
            expect(await token.ownerOf(0)).to.equal(addr2.address);
        });

        it("Should set and check operator approval", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.mint(addr1.address);
            await token.connect(addr1).setApprovalForAll(addr2.address, true);
            expect(await token.isApprovedForAll(addr1.address, addr2.address)).to.be.true;

            await token.connect(addr2).transferFrom(addr1.address, addr2.address, 0);
            expect(await token.ownerOf(0)).to.equal(addr2.address);
        });

        it("Should fail if trying to transfer without approval", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.mint(addr1.address);
            await expect(token.connect(addr2).transferFrom(addr1.address, addr2.address, 0)).to.be.revertedWithCustomError(token, "ERC721InsufficientApproval");
        });
    });

    describe("Base URI", function () {
        it("Should return correct token URI after minting", async function () {
            const { token, owner, addr1 } = await loadFixture(deployTokenFixture);

            await token.mint(addr1.address);
            expect(await token.tokenURI(0)).to.equal("https://baseuri.com/0");
        });
    });
});
