import { Address, ethereum, BigInt } from "@graphprotocol/graph-ts";
import { test, assert, newMockEvent, createMockedFunction, describe, afterAll, clearStore, beforeEach, mockIpfsFile } from "matchstick-as";
import { handleAwardItem, handleBurn, handleTransfer, handleApproval, handleApprovalForAll } from "../src/erc-721";
import { AwardItem, Burn, Transfer as TransferEntity, Approval as ApprovalEntity, ApprovalForAll as ApprovalForAllEntity, NFT, User } from "../fix-generated/schema";
import { loadUser, ZERO_ADDRESS } from "../src/helpers";
import { createApprovalEvent, createApprovalForAllEvent, createAwardItemEvent, createBurnEvent, createTransferEvent } from "./erc-721-utils";


// Mock a generic tokenId and cid
const mockTokenId = BigInt.fromI32(0);
const mockCid = "QmQQrY2RcZCQ1mh8tMV4DVQL2AVcK4Pheu3pvD3TNtvQZu";
let address = Address.fromString('0x0000000000000000000000000000000000000001')

// Create a mock IPFS service

describe("ERC-721 Subgraph", () => {
  beforeEach(() => {
    // Set up mock contract calls
    createMockedFunction(address, 'name', 'name():(string)').reverts()
    createMockedFunction(address, 'symbol', 'symbol():(string)').reverts()
    mockIpfsFile(mockCid, './tests/mock-metadata.json')
  });

  afterAll(() => {
    clearStore()
  });

  test("Test handleAwardItem", () => {
    let recipient = Address.fromString('0x1111111111111111111111111111111111111111')
    let event = createAwardItemEvent(recipient, mockCid, mockTokenId);
    event.address = address

    handleAwardItem(event);

    let nftId = "0x0000000000000000000000000000000000000001-" + mockTokenId.toString();
    let entity = AwardItem.load(event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString());

    assert.assertNotNull(entity);
    assert.stringEquals(entity!.recipient.toHex(), "0x1111111111111111111111111111111111111111");
    assert.stringEquals(entity!.cid, mockCid);
    assert.bigIntEquals(entity!.tokenId, mockTokenId);

    const nft_entity = NFT.load(nftId);
    assert.assertNotNull(nft_entity);
    assert.bigIntEquals(nft_entity!.tokenId, mockTokenId);
    assert.stringEquals(nft_entity!.owner, '0x1111111111111111111111111111111111111111');
    assert.stringEquals(nft_entity!.collection, "0x0000000000000000000000000000000000000001");
  });

  test("Test handleTransfer", () => {
    let from = Address.fromString('0x1111111111111111111111111111111111111111')
    let to = Address.fromString('0x2222222222222222222222222222222222222222')
    let transfer_event = createTransferEvent(from, to, mockTokenId);
    transfer_event.address = address

    handleTransfer(transfer_event);

    let nftId = "0x0000000000000000000000000000000000000001-" + mockTokenId.toString();
    let entity = TransferEntity.load(transfer_event.transaction.hash.toHex() + "-" + transfer_event.logIndex.toI32().toString());

    assert.assertNotNull(entity);
    assert.stringEquals(entity!.from.toHex(), "0x1111111111111111111111111111111111111111");
    assert.stringEquals(entity!.to.toHex(), "0x2222222222222222222222222222222222222222");
    assert.bigIntEquals(entity!.tokenId, mockTokenId);
    assert.stringEquals(entity!.nft, nftId);

    let nft_entity = NFT.load(nftId);
    assert.assertNotNull(nft_entity);
    assert.stringEquals(nft_entity!.owner, '0x2222222222222222222222222222222222222222');
    assert.stringEquals(nft_entity!.collection, "0x0000000000000000000000000000000000000001");

    let from_entity = loadUser(from);
    assert.assertNotNull(from_entity);
    assert.bigIntEquals(from_entity!.numberTokens, BigInt.fromI32(0));

    let to_entity = loadUser(to);
    assert.assertNotNull(to_entity);
    assert.bigIntEquals(to_entity!.numberTokens, BigInt.fromI32(1));
  });

  test("Test handleBurn", () => {
    let recipient = Address.fromString('0x2222222222222222222222222222222222222222')

    let burn_event = createBurnEvent(mockTokenId);
    burn_event.address = address

    handleBurn(burn_event);

    let nftId = "0x0000000000000000000000000000000000000001-" + mockTokenId.toString();
    let entity = Burn.load(burn_event.transaction.hash.toHex() + "-" + burn_event.logIndex.toI32().toString());

    assert.assertNotNull(entity);
    assert.bigIntEquals(entity!.tokenId, mockTokenId);
    assert.stringEquals(entity!.nft, nftId);

    let nft_entity = NFT.load(nftId);
    assert.assertNotNull(nft_entity);
    assert.stringEquals(nft_entity!.owner, '0x0000000000000000000000000000000000000000');
    assert.stringEquals(nft_entity!.collection, "0x0000000000000000000000000000000000000001");

    let user_entity = loadUser(recipient);
    assert.assertNotNull(user_entity);
    assert.bigIntEquals(user_entity!.numberTokens, BigInt.fromI32(0));
  });

  test("Test handleApproval", () => {
    let owner = Address.fromString('0x2222222222222222222222222222222222222222')
    let approved = Address.fromString('0x1111111111111111111111111111111111111111')

    let newApprovalEvent = createApprovalEvent(owner, approved, mockTokenId);
    newApprovalEvent.address = address

    handleApproval(newApprovalEvent);

    let entity = ApprovalEntity.load(newApprovalEvent.transaction.hash.toHex() + "-" + newApprovalEvent.logIndex.toI32().toString());

    assert.assertNotNull(entity);
    assert.stringEquals(entity!.owner, "0x2222222222222222222222222222222222222222");
    assert.stringEquals(entity!.approved, "0x1111111111111111111111111111111111111111");
    assert.bigIntEquals(entity!.tokenId, mockTokenId);

    let owner_entity = loadUser(owner);
    assert.assertNotNull(owner_entity);
    assert.stringEquals(owner_entity!.approvalsGiven._id, owner_entity!.id);

    let approved_entity = loadUser(approved);
    assert.assertNotNull(approved_entity);
    assert.stringEquals(approved_entity!.approvalsReceived._id, approved_entity!.id);
  });

  test("Test handleApprovalForAll", () => {
    let owner = Address.fromString('0x2222222222222222222222222222222222222222')
    let spender = Address.fromString('0x1111111111111111111111111111111111111111')
    let newApprovalForAllEvent = createApprovalForAllEvent(owner, spender, true);
    newApprovalForAllEvent.address = address

    handleApprovalForAll(newApprovalForAllEvent);

    let entity = ApprovalForAllEntity.load(newApprovalForAllEvent.transaction.hash.toHex() + "-" + newApprovalForAllEvent.logIndex.toI32().toString());

    assert.assertNotNull(entity);
    assert.stringEquals(entity!.owner, "0x2222222222222222222222222222222222222222");
    assert.stringEquals(entity!.operator, "0x1111111111111111111111111111111111111111");
    assert.booleanEquals(entity!.approved, true);

    let owner_entity = loadUser(owner);
    assert.assertNotNull(owner_entity);
    assert.stringEquals(owner_entity!.approvalForAllGiven._id, owner_entity!.id);

    let spender_entity = loadUser(spender);
    assert.assertNotNull(spender_entity);
    assert.stringEquals(spender_entity!.approvalForAllReceived._id, spender_entity!.id);
  });
})