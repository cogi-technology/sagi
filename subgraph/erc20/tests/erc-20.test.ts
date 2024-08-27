import { Address, BigInt, ethereum } from "@graphprotocol/graph-ts";
import { afterAll, assert, beforeEach, clearStore, createMockedFunction, describe, newMockEvent, test } from "matchstick-as";
import { handleTransfer, handleApproval } from "../src/erc-20";
import { Approval, Transfer } from "../fix-generated/schema";
import { loadCollection, loadBalance, loadUser } from "../src/helpers";
import { createTransferEvent, createApprovalEvent, createTransferOperatorEvent } from "./erc-20-utils";

let address = Address.fromString('0x0000000000000000000000000000000000000001')

describe("ERC-20 Subgraph", () => {
  beforeEach(() => {
    // Set up mock contract calls
    createMockedFunction(address, 'name', 'name():(string)').reverts()
    createMockedFunction(address, 'symbol', 'symbol():(string)').reverts()
    createMockedFunction(address, 'totalSupply', 'totalSupply():(uint256)').reverts()
  });

  afterAll(() => {
    clearStore()
  });

  test('fetchName returns "unknown" when contract call reverts', () => {
    const collection = loadCollection(address)
    assert.stringEquals(collection.symbol, 'unknown')
    assert.stringEquals(collection.name, 'unknown')
    assert.bigIntEquals(collection.totalSupply, BigInt.fromI32(0))
  })

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

    const erc20FromBalance = loadBalance(collection, from_instance)
    assert.assertNotNull(erc20FromBalance)
    assert.bigIntEquals(erc20FromBalance.balance, BigInt.fromI32(0))

    const erc20ToBalance = loadBalance(collection, to_instance)
    assert.assertNotNull(erc20ToBalance)
    assert.bigIntEquals(erc20ToBalance.balance, BigInt.fromI32(100))
  });

  test("handleApproval creates and updates Approval entity", () => {
    // Create mock approval event
    let event = createApprovalEvent(Address.fromString("0x1111111111111111111111111111111111111111"), Address.fromString("0x2222222222222222222222222222222222222222"), BigInt.fromI32(100));
    event.address = address

    // Call handleApproval
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

  test("handleApproval then handleTransfer updates allowances correctly", () => {
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

    const erc20FromBalance = loadBalance(collection, from_instance)
    assert.assertNotNull(erc20FromBalance)
    assert.bigIntEquals(erc20FromBalance.balance, BigInt.fromI32(0))

    const erc20ToBalance = loadBalance(collection, to_instance)
    assert.assertNotNull(erc20ToBalance)
    assert.bigIntEquals(erc20ToBalance.balance, BigInt.fromI32(100))
  });

  // Add more test cases for different scenarios and edge cases
});
