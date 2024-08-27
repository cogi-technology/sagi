import { Address, BigInt, ethereum } from "@graphprotocol/graph-ts";
import { afterAll, assert, beforeEach, clearStore, createMockedFunction, describe, newMockEvent, test } from "matchstick-as";
import {
  handleApproval,
  handleApprovalForAll,
  handleBurn,
  handleERC1155SetTransferExempt,
  handleMint,
  handleTransfer,
  handleTransferBatch,
  handleTransferSingle
} from "../src/erc-404";
import { Approval, ApprovalForAll, Burn, ERC1155Balance, Mint, Transfer } from "../fix-generated/schema";
import { loadCollection, loadErc1155BalanceOfBatch, loadErc20Balance, loadUser } from "../src/helpers";
import { createApprovalEvent, createApprovalForAllEvent, createBurnEvent, createERC1155SetTransferExemptEvent, createMintEvent, createTransferBatchEvent, createTransferEvent, createTransferOperatorEvent, createTransferSingleEvent } from "./erc-404-utils";

// Helper function to create mock events
function createMockEvent<T extends ethereum.Event>(eventType: string): T {
  let mockEvent = newMockEvent();
  mockEvent.address = Address.fromString("0x0000000000000000000000000000000000000001");
  return mockEvent as T;
}

let address = Address.fromString('0x0000000000000000000000000000000000000001')

describe("ERC-404 Subgraph", () => {
  beforeEach(() => {
    createMockedFunction(address, 'name', 'name():(string)').reverts()
    createMockedFunction(address, 'symbol', 'symbol():(string)').reverts()
    createMockedFunction(address, 'totalSupply', 'totalSupply():(uint256)').reverts()
  });

  afterAll(() => {
    clearStore()
  })

  test('fetchName returns "unknown" when contract call reverts', () => {
    const collection = loadCollection(address)
    assert.stringEquals(collection.symbol, 'unknown')
    assert.stringEquals(collection.name, 'unknown')
    assert.bigIntEquals(collection.totalSupply, BigInt.fromI32(0))
  })

  test("handleApproval creates and updates Approval entity", () => {
    let event = createApprovalEvent(Address.fromString("0x1111111111111111111111111111111111111111"), Address.fromString("0x2222222222222222222222222222222222222222"), BigInt.fromI32(100));
    event.address = address

    handleApproval(event);
    // Verify that the Approval entity was created and updated correctly
    const approvalId = "0x0000000000000000000000000000000000000001-0x1111111111111111111111111111111111111111-0x2222222222222222222222222222222222222222";
    const approval = Approval.load(approvalId);

    // Add assertions to check if the Approval entity was created and updated correctly
    assert.assertNotNull(approval);
    assert.stringEquals(approval!.owner, "0x1111111111111111111111111111111111111111");
    assert.stringEquals(approval!.spender, "0x2222222222222222222222222222222222222222");
    assert.bigIntEquals(approval!.value, BigInt.fromI32(100));
    assert.bigIntEquals(approval!.remaining_allowance, BigInt.fromI32(100));
    assert.stringEquals(approval!.collection, "0x0000000000000000000000000000000000000001");
  });

  test("handleApprovalForAll creates and updates ApprovalForAll entity", () => {
    let event = createApprovalForAllEvent(Address.fromString("0x1111111111111111111111111111111111111111"), Address.fromString("0x2222222222222222222222222222222222222222"), true);
    event.address = address

    handleApprovalForAll(event);

    // Verify that the Approval entity was created and updated correctly
    const id = "0x0000000000000000000000000000000000000001-0x1111111111111111111111111111111111111111-0x2222222222222222222222222222222222222222";
    const approvalForAll = ApprovalForAll.load(id);

    // Add assertions to check if the ApprovalForAll entity was created and updated correctly
    assert.assertNotNull(approvalForAll);
    assert.stringEquals(approvalForAll!.account, "0x1111111111111111111111111111111111111111");
    assert.stringEquals(approvalForAll!.operator, "0x2222222222222222222222222222222222222222");
    assert.booleanEquals(approvalForAll!.approved, true);
    assert.stringEquals(approvalForAll!.collection, "0x0000000000000000000000000000000000000001");
  });

  test("handleBurn creates Burn entity and updates balances", () => {
    let account = Address.fromString("0x1111111111111111111111111111111111111111")
    let event = createBurnEvent(account, BigInt.fromI32(100));
    event.address = address

    let mock_ids = [BigInt.fromI32(1), BigInt.fromI32(2), BigInt.fromI32(3)]
    let mock_balances = [BigInt.fromI32(0), BigInt.fromI32(1), BigInt.fromI32(2)]

    // generate mock onchain data for functions: balanceOf(fetchBalanceOf), ids, balanceOfBatch(fetchBalanceOfBatch)
    createMockedFunction(address, 'balanceOf', 'balanceOf(address):(uint256)').withArgs([ethereum.Value.fromAddress(account)]).returns([ethereum.Value.fromI32(0)])
    createMockedFunction(address, 'ids', 'ids():(uint256[])').returns([ethereum.Value.fromUnsignedBigIntArray(mock_ids)])
    createMockedFunction(address, 'balanceOfBatch', 'balanceOfBatch(address[],uint256[]):(uint256[])')
      .withArgs(
        [ethereum.Value.fromAddressArray([account, account, account]),
        ethereum.Value.fromUnsignedBigIntArray(mock_ids)])
      .returns([ethereum.Value.fromUnsignedBigIntArray(mock_balances)])

    handleBurn(event);

    // Add assertions to check if the Burn entity was created and balances were updated correctly
    const id = event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString()
    const burn = Burn.load(id);

    // Add assertions to check if the Mint entity was created and balances were updated correctly
    assert.assertNotNull(burn);
    assert.stringEquals(burn!.account, account.toHex())
    assert.bigIntEquals(burn!.amount, BigInt.fromI32(100))
    assert.stringEquals(burn!.collection, "0x0000000000000000000000000000000000000001");

    const collection = loadCollection(Address.fromString(burn!.collection))
    const account_instance = loadUser(account)

    const erc20Balance = loadErc20Balance(collection, account_instance)
    assert.bigIntEquals(erc20Balance.balance, BigInt.fromI32(0))

    const erc1155BatchBalance = loadErc1155BalanceOfBatch(collection, account_instance)
    for (let i = 0; i < erc1155BatchBalance.batchBalance.length; i++) {
      let erc1155Balance = ERC1155Balance.load(erc1155BatchBalance.batchBalance[i])

      assert.assertNotNull(erc1155Balance)
      assert.bigIntEquals(erc1155Balance!.balance, mock_balances[i])
    }
  });

  test("handleERC1155SetTransferExempt updates User entity", () => {
    let event = createERC1155SetTransferExemptEvent(Address.fromString("0x1111111111111111111111111111111111111111"), true);

    handleERC1155SetTransferExempt(event);

    // Add assertions to check if the User entity was updated correctly
    const account = loadUser(event.params.account)
    assert.booleanEquals(account!.isErc1155TransferExempt, true);
  });

  test("handleMint creates Mint entity and updates balances", () => {
    const account = Address.fromString("0x1111111111111111111111111111111111111111")
    let event = createMintEvent(account, BigInt.fromI32(100));
    event.address = address

    let mock_ids = [BigInt.fromI32(1), BigInt.fromI32(2), BigInt.fromI32(3)]
    let mock_balances = [BigInt.fromI32(10), BigInt.fromI32(20), BigInt.fromI32(30)]

    createMockedFunction(address, 'balanceOf', 'balanceOf(address):(uint256)').withArgs([ethereum.Value.fromAddress(account)]).returns([ethereum.Value.fromI32(100)])
    createMockedFunction(address, 'ids', 'ids():(uint256[])').returns([ethereum.Value.fromUnsignedBigIntArray(mock_ids)])
    createMockedFunction(address, 'balanceOfBatch', 'balanceOfBatch(address[],uint256[]):(uint256[])')
      .withArgs(
        [ethereum.Value.fromAddressArray([account, account, account]),
        ethereum.Value.fromUnsignedBigIntArray(mock_ids)])
      .returns([ethereum.Value.fromUnsignedBigIntArray(mock_balances)])

    handleMint(event);

    const id = event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString()
    const mint = Mint.load(id);

    // Add assertions to check if the Mint entity was created and balances were updated correctly
    assert.assertNotNull(mint);
    assert.stringEquals(mint!.account, account.toHex())
    assert.bigIntEquals(mint!.amount, BigInt.fromI32(100))
    assert.stringEquals(mint!.collection, "0x0000000000000000000000000000000000000001");

    const collection = loadCollection(Address.fromString(mint!.collection))
    const account_instance = loadUser(account)

    const erc20Balance = loadErc20Balance(collection, account_instance)
    assert.bigIntEquals(erc20Balance.balance, BigInt.fromI32(100))

    const erc1155BatchBalance = loadErc1155BalanceOfBatch(collection, account_instance)
    for (let i = 0; i < erc1155BatchBalance.batchBalance.length; i++) {
      let erc1155Balance = ERC1155Balance.load(erc1155BatchBalance.batchBalance[i])

      assert.assertNotNull(erc1155Balance)
      assert.bigIntEquals(erc1155Balance!.balance, mock_balances[i])
    }
  });

  test("handleTransfer when from is operator and creates Transfer entity and updates balances", () => {
    let operator = Address.fromString("0x1111111111111111111111111111111111111111");
    let from = Address.fromString("0x1111111111111111111111111111111111111111");
    let to = Address.fromString("0x2222222222222222222222222222222222222222");

    let event = createTransferOperatorEvent(
      operator,
      from,
      to,
      BigInt.fromI32(100)
    );
    event.address = address

    createMockedFunction(address, 'balanceOf', 'balanceOf(address):(uint256)').withArgs([ethereum.Value.fromAddress(from)]).returns([ethereum.Value.fromI32(0)])
    createMockedFunction(address, 'balanceOf', 'balanceOf(address):(uint256)').withArgs([ethereum.Value.fromAddress(to)]).returns([ethereum.Value.fromI32(100)])

    handleTransfer(event);

    let id = event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString()
    const transfer = Transfer.load(id);

    // Add assertions to check if the Transfer entity was created and balances were updated correctly
    assert.assertNotNull(transfer);
    assert.stringEquals(transfer!.operator, operator.toHex())
    assert.stringEquals(transfer!.from, from.toHex())
    assert.stringEquals(transfer!.to, to.toHex())
    assert.bigIntEquals(transfer!.value, BigInt.fromI32(100))
    assert.stringEquals(transfer!.collection, "0x0000000000000000000000000000000000000001");

    const collection = loadCollection(Address.fromString(transfer!.collection))
    const from_instance = loadUser(from)
    const to_instance = loadUser(to)

    const erc20FromBalance = loadErc20Balance(collection, from_instance)
    assert.assertNotNull(erc20FromBalance)
    assert.bigIntEquals(erc20FromBalance.balance, BigInt.fromI32(0))

    const erc20ToBalance = loadErc20Balance(collection, to_instance)
    assert.assertNotNull(erc20ToBalance)
    assert.bigIntEquals(erc20ToBalance.balance, BigInt.fromI32(100))
  });

  test("handleApproval then handleTransfer creates Approval, Transfer entities and updates balances and allowances", () => {
    let operator = Address.fromString("0x3333333333333333333333333333333333333333");
    let from = Address.fromString("0x1111111111111111111111111111111111111111");
    let to = Address.fromString("0x2222222222222222222222222222222222222222");

    let approval_event = createApprovalEvent(
      from,
      operator,
      BigInt.fromI32(100)
    );
    approval_event.address = address

    handleApproval(approval_event);

    // Verify that the Approval entity was created and updated correctly
    const approvalId = "0x0000000000000000000000000000000000000001-0x1111111111111111111111111111111111111111-0x3333333333333333333333333333333333333333";
    const approval_before_transfer = Approval.load(approvalId);

    // Add assertions to check if the Approval entity was created and updated correctly
    assert.assertNotNull(approval_before_transfer);
    assert.stringEquals(approval_before_transfer!.owner, "0x1111111111111111111111111111111111111111");
    assert.stringEquals(approval_before_transfer!.spender, "0x3333333333333333333333333333333333333333");
    assert.bigIntEquals(approval_before_transfer!.value, BigInt.fromI32(100));
    assert.bigIntEquals(approval_before_transfer!.remaining_allowance, BigInt.fromI32(100));
    assert.stringEquals(approval_before_transfer!.collection, "0x0000000000000000000000000000000000000001");

    let transfer_operator_event = createTransferOperatorEvent(
      operator,
      from,
      to,
      BigInt.fromI32(100)
    );
    transfer_operator_event.address = address

    createMockedFunction(address, 'balanceOf', 'balanceOf(address):(uint256)').withArgs([ethereum.Value.fromAddress(from)]).returns([ethereum.Value.fromI32(0)])
    createMockedFunction(address, 'balanceOf', 'balanceOf(address):(uint256)').withArgs([ethereum.Value.fromAddress(to)]).returns([ethereum.Value.fromI32(100)])

    handleTransfer(transfer_operator_event);

    let id = transfer_operator_event.transaction.hash.toHex() + "-" + transfer_operator_event.logIndex.toI32().toString()
    const transfer = Transfer.load(id);

    // Add assertions to check if the Transfer entity was created and balances were updated correctly
    assert.assertNotNull(transfer);
    assert.stringEquals(transfer!.operator, operator.toHex())
    assert.stringEquals(transfer!.from, from.toHex())
    assert.stringEquals(transfer!.to, to.toHex())
    assert.bigIntEquals(transfer!.value, BigInt.fromI32(100))
    assert.stringEquals(transfer!.collection, "0x0000000000000000000000000000000000000001");

    const approval_after_transfer = Approval.load(approvalId);
    assert.bigIntEquals(approval_after_transfer!.remaining_allowance, BigInt.fromI32(0));

    const collection = loadCollection(Address.fromString(transfer!.collection))
    const from_instance = loadUser(from)
    const to_instance = loadUser(to)

    const erc20FromBalance = loadErc20Balance(collection, from_instance)
    assert.assertNotNull(erc20FromBalance)
    assert.bigIntEquals(erc20FromBalance.balance, BigInt.fromI32(0))

    const erc20ToBalance = loadErc20Balance(collection, to_instance)
    assert.assertNotNull(erc20ToBalance)
    assert.bigIntEquals(erc20ToBalance.balance, BigInt.fromI32(100))
  });

  test("handleTransferBatch creates TransferBatch entity and updates balances", () => {
    let operator = Address.fromString("0x3333333333333333333333333333333333333333");
    let from = Address.fromString("0x1111111111111111111111111111111111111111");
    let to = Address.fromString("0x2222222222222222222222222222222222222222");
    let ids = [BigInt.fromI32(1), BigInt.fromI32(2), BigInt.fromI32(3)];
    let values = [BigInt.fromI32(10), BigInt.fromI32(20), BigInt.fromI32(30)];

    let event = createTransferBatchEvent(
      operator,
      from,
      to,
      ids,
      values
    );
    event.address = address

    let from_mock_balances = [BigInt.fromI32(0), BigInt.fromI32(0), BigInt.fromI32(0)]
    let to_mock_balances = [BigInt.fromI32(10), BigInt.fromI32(20), BigInt.fromI32(30)]

    createMockedFunction(address, 'ids', 'ids():(uint256[])').returns([ethereum.Value.fromUnsignedBigIntArray(ids)])
    createMockedFunction(address, 'balanceOfBatch', 'balanceOfBatch(address[],uint256[]):(uint256[])')
      .withArgs([ethereum.Value.fromAddressArray([from, from, from]), ethereum.Value.fromUnsignedBigIntArray(ids)])
      .returns([ethereum.Value.fromUnsignedBigIntArray(from_mock_balances)]);
    createMockedFunction(address, 'balanceOfBatch', 'balanceOfBatch(address[],uint256[]):(uint256[])')
      .withArgs([ethereum.Value.fromAddressArray([to, to, to]), ethereum.Value.fromUnsignedBigIntArray(ids)])
      .returns([ethereum.Value.fromUnsignedBigIntArray(to_mock_balances)]);

    handleTransferBatch(event);

    // Add assertions to check if the TransferBatch entity was created and balances were updated correctly
    let collection = loadCollection(event.address)
    let from_instance = loadUser(from)
    let to_instance = loadUser(to)

    const fromErc1155BatchBalance = loadErc1155BalanceOfBatch(collection, from_instance)
    for (let i = 0; i < fromErc1155BatchBalance.batchBalance.length; i++) {
      let erc1155Balance = ERC1155Balance.load(fromErc1155BatchBalance.batchBalance[i])

      assert.assertNotNull(erc1155Balance)
      assert.bigIntEquals(erc1155Balance!.balance, from_mock_balances[i])
    }

    const toErc1155BatchBalance = loadErc1155BalanceOfBatch(collection, to_instance)
    for (let i = 0; i < toErc1155BatchBalance.batchBalance.length; i++) {
      let erc1155Balance = ERC1155Balance.load(toErc1155BatchBalance.batchBalance[i])

      assert.assertNotNull(erc1155Balance)
      assert.bigIntEquals(erc1155Balance!.balance, to_mock_balances[i])
    }
  });

  test("handleTransferSingle creates TransferSingle entity and updates balances", () => {
    let operator = Address.fromString("0x3333333333333333333333333333333333333333");
    let from = Address.fromString("0x1111111111111111111111111111111111111111");
    let to = Address.fromString("0x2222222222222222222222222222222222222222");
    let id = BigInt.fromI32(0)
    let value = BigInt.fromI32(100)

    let event = createTransferSingleEvent(
      operator,
      from,
      to,
      id,
      value
    );
    event.address = address

    let from_mock_balances = [BigInt.fromI32(0)]
    let to_mock_balances = [BigInt.fromI32(100)]

    createMockedFunction(address, 'ids', 'ids():(uint256[])').returns([ethereum.Value.fromUnsignedBigIntArray([id])])
    createMockedFunction(address, 'balanceOfBatch', 'balanceOfBatch(address[],uint256[]):(uint256[])')
      .withArgs([ethereum.Value.fromAddressArray([from]), ethereum.Value.fromUnsignedBigIntArray([id])])
      .returns([ethereum.Value.fromUnsignedBigIntArray(from_mock_balances)]);
    createMockedFunction(address, 'balanceOfBatch', 'balanceOfBatch(address[],uint256[]):(uint256[])')
      .withArgs([ethereum.Value.fromAddressArray([to]), ethereum.Value.fromUnsignedBigIntArray([id])])
      .returns([ethereum.Value.fromUnsignedBigIntArray(to_mock_balances)]);

    handleTransferSingle(event);

    // Add assertions to check if the TransferSingle entity was created and balances were updated correctly
    let collection = loadCollection(event.address)
    let from_instance = loadUser(from)
    let to_instance = loadUser(to)

    const fromErc1155BatchBalance = loadErc1155BalanceOfBatch(collection, from_instance)
    for (let i = 0; i < fromErc1155BatchBalance.batchBalance.length; i++) {
      let erc1155Balance = ERC1155Balance.load(fromErc1155BatchBalance.batchBalance[i])

      assert.assertNotNull(erc1155Balance)
      assert.bigIntEquals(erc1155Balance!.balance, from_mock_balances[i])
    }

    const toErc1155BatchBalance = loadErc1155BalanceOfBatch(collection, to_instance)
    for (let i = 0; i < toErc1155BatchBalance.batchBalance.length; i++) {
      let erc1155Balance = ERC1155Balance.load(toErc1155BatchBalance.batchBalance[i])

      assert.assertNotNull(erc1155Balance)
      assert.bigIntEquals(erc1155Balance!.balance, to_mock_balances[i])
    }
  });
});
