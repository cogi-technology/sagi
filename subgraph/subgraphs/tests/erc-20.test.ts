import {
  assert,
  describe,
  test,
  clearStore,
  beforeAll,
  afterAll,
  logEntity
} from "matchstick-as/assembly/index"
import { Address, BigInt } from "@graphprotocol/graph-ts"
import { Erc20Approval } from "../generated/schema"
import { Approval as ApprovalEvent } from "../generated/erc20/erc20"
import { handleApproval } from "../src/erc-20"
import { createApprovalEvent } from "./erc-20-utils"

// Tests structure (matchstick-as >=0.5.0)
// https://thegraph.com/docs/en/developer/matchstick/#tests-structure-0-5-0

describe("Describe entity assertions", () => {
  beforeAll(() => {
    let owner = Address.fromString("0x0000000000000000000000000000000000000001")
    let spender = Address.fromString(
      "0x0000000000000000000000000000000000000001"
    )
    let value = BigInt.fromI32(234)
    let newApprovalEvent = createApprovalEvent(owner, spender, value)

    handleApproval(newApprovalEvent)
  })

  afterAll(() => {
    clearStore()
  })

  // For more test scenarios, see:
  // https://thegraph.com/docs/en/developer/matchstick/#write-a-unit-test

  test("Approval created and stored", () => {
    assert.entityCount("Erc20Approval", 1)

    // 0xa16081f360e3847006db660bae1c6d1b2e17ec2a is the default address used in newMockEvent() function
    assert.fieldEquals(
      "Erc20Approval",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a01000000",
      "owner",
      "0x0000000000000000000000000000000000000001"
    )
    assert.fieldEquals(
      "Erc20Approval",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a01000000",
      "spender",
      "0x0000000000000000000000000000000000000001"
    )
    assert.fieldEquals(
      "Erc20Approval",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a01000000",
      "value",
      "234"
    )

    // More assert options:
    // https://thegraph.com/docs/en/developer/matchstick/#asserts
  })
})
