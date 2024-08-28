import { expect } from "chai";
import "@openzeppelin/hardhat-upgrades";
import { ethers } from "hardhat";
import {
    loadFixture,
} from "@nomicfoundation/hardhat-network-helpers";


describe("TokenERC721", function () {
    async function deployTokenFixture() {
        const [owner, addr1, addr2, addr3] = await ethers.getSigners();
        const ERC20 = await ethers.getContractFactory("TokenERC20");
        const erc20 = await ERC20.deploy(owner.address, "TestTokenERC20", "TTE20", ethers.parseEther("1000000000"));

        const Token = await ethers.getContractFactory("TokenERC721");
        const token = await Token.deploy(owner.address, "MyToken", "MTK", "https://base.uri/", await erc20.getAddress());
        await token.waitForDeployment();

        return { token, owner, addr1, addr2, addr3 };
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

    describe("awardItem", function () {
        it("Should awardItem a new token with correct URI", async function () {
            const { token: token, owner, addr1 } = await loadFixture(deployTokenFixture);
            const cid = "unique_cid";

            await token.awardItem(addr1.address, cid);

            const tokenId = 0;
            expect(await token.balanceOf(addr1.address)).to.equal(1);
            expect(await token.ownerOf(0)).to.equal(addr1.address);
            expect(await token.tokenURI(tokenId)).to.equal(`https://base.uri/${cid}`);
        });

        it("Should revert if the cid has already been used", async function () {
            const { token, owner, addr1 } = await loadFixture(deployTokenFixture);
            const cid = "unique_cid";

            await token.awardItem(addr1.address, cid);
            await expect(token.awardItem(addr1.address, cid)).to.be.revertedWith("_awardItem: cid invalid");
        });

        it("Should fail if minting is attempted by non-owner", async function () {
            const { token, addr1 } = await loadFixture(deployTokenFixture);
            const cid = "unique_cid";

            await expect(token.connect(addr1).awardItem(addr1.address, cid)).to.be.revertedWith("ERC721: must have minter role to awardItem");
        });
    });

    describe("awardItems", function () {
        it("Should awardItem multiple tokens to different addresses", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);
            const cids = ["cid1", "cid2"];
            const recipients = [addr1.address, addr2.address];

            await token.awardItems(recipients, cids);

            expect(await token.balanceOf(addr1.address)).to.equal(1);
            expect(await token.balanceOf(addr2.address)).to.equal(1);

            expect(await token.tokenURI(0)).to.equal(`https://base.uri/cid1`);
            expect(await token.tokenURI(1)).to.equal(`https://base.uri/cid2`);
        });

        it("Should revert if any cid has already been used", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);
            const cids = ["cid1", "cid2"];
            const recipients = [addr1.address, addr2.address];

            await token.awardItem(addr1.address, "cid1");

            await expect(token.awardItems(recipients, cids)).to.be.revertedWith("_awardItem: cid invalid");
        });

        it("Should revert if recipients and cids arrays have different lengths", async function () {
            const { token, owner, addr1 } = await loadFixture(deployTokenFixture);
            const cids = ["cid1"];
            const recipients = [addr1.address, addr1.address, addr1.address];

            await expect(token.awardItems(recipients, cids)).to.be.reverted;
        });
    });

    describe("force_burns", function () {
        it("Should burn multiple tokens and clear their CIDs", async function () {
            const { token, owner, addr1, addr2, addr3 } = await loadFixture(deployTokenFixture);

            // Mint multiple tokens
            await token.awardItem(addr1.address, "cid1");
            await token.awardItem(addr2.address, "cid2");
            await token.awardItem(addr3.address, "cid3");

            // Token IDs to be burned
            const tokenIds = [0, 1, 2];

            // Force burn multiple tokens
            await token.force_burns(tokenIds);

            // Check that the tokens were burned
            for (let tokenId of tokenIds) {
                await expect(token.tokenURI(tokenId)).to.be.revertedWithCustomError(token, "ERC721NonexistentToken");
            }

            // Check that the CIDs are cleared
            expect(await token.cids("cid1")).to.equal(0);
            expect(await token.cids("cid2")).to.equal(0);
            expect(await token.cids("cid3")).to.equal(0);
        });

        it("Should revert if trying to force burn a non-existent token", async function () {
            const { token, owner } = await loadFixture(deployTokenFixture);

            // Try to burn non-existent tokens
            await expect(token.force_burns([9999])).to.be.revertedWithCustomError(token, "ERC721NonexistentToken");
        });

        it("Should revert if trying to force burn a token with an invalid CID", async function () {
            const { token, owner, addr1 } = await loadFixture(deployTokenFixture);

            // Mint and then manually invalidate CID by burning
            await token.awardItem(addr1.address, "cid1");
            await token.force_burns([0]);

            // Try to burn again which should fail due to invalid CID
            await expect(token.force_burns([0])).to.be.revertedWithCustomError(token, "ERC721NonexistentToken");
        });
    });

    describe("tokenURI", function () {
        it("Should return the correct token URI", async function () {
            const { token, owner, addr1 } = await loadFixture(deployTokenFixture);
            const cid = "unique_cid";

            await token.awardItem(addr1.address, cid);

            const tokenId = 0;
            expect(await token.tokenURI(tokenId)).to.equal(`https://base.uri/${cid}`);
        });

        it("Should revert if the token does not exist", async function () {
            const { token } = await loadFixture(deployTokenFixture);
            await expect(token.tokenURI(9999)).to.be.revertedWithCustomError(token, "ERC721NonexistentToken");
        });
    });

    describe("Transfers", function () {
        it("Should transfer token between accounts", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.awardItem(addr1.address, "unique_cid");
            await token.connect(addr1).transferFrom(addr1.address, addr2.address, 0);
            expect(await token.ownerOf(0)).to.equal(addr2.address);
        });

        it("Should fail if transfer is attempted by non-owner", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.awardItem(addr1.address, "cid1");
            await expect(token.connect(addr2).transferFrom(addr1.address, addr2.address, 0)).to.be.revertedWithCustomError(token, "ERC721InsufficientApproval");
        });
    });

    describe("Approvals", function () {
        it("Should approve and then transfer a token by an approved account", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.awardItem(addr1.address, "cid1");
            await token.connect(addr1).approve(addr2.address, 0);
            await token.connect(addr2).transferFrom(addr1.address, addr2.address, 0);
            expect(await token.ownerOf(0)).to.equal(addr2.address);
        });

        it("Should set and check operator approval", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.awardItem(addr1.address, "cid1");
            await token.connect(addr1).setApprovalForAll(addr2.address, true);
            expect(await token.isApprovedForAll(addr1.address, addr2.address)).to.be.true;

            await token.connect(addr2).transferFrom(addr1.address, addr2.address, 0);
            expect(await token.ownerOf(0)).to.equal(addr2.address);
        });

        it("Should fail if trying to transfer without approval", async function () {
            const { token, owner, addr1, addr2 } = await loadFixture(deployTokenFixture);

            await token.awardItem(addr1.address, "cid1");
            await expect(token.connect(addr2).transferFrom(addr1.address, addr2.address, 0)).to.be.revertedWithCustomError(token, "ERC721InsufficientApproval");
        });
    });

    describe("Base URI", function () {
        it("Should return correct token URI after minting", async function () {
            const { token, owner, addr1 } = await loadFixture(deployTokenFixture);

            await token.awardItem(addr1.address, "cid1");
            expect(await token.tokenURI(0)).to.equal("https://base.uri/cid1");
        });
    });
});
