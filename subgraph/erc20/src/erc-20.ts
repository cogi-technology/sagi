import { Address } from "@graphprotocol/graph-ts"
import {
  TransferOperator as TransferOperatorEvent,
  Approval as ApprovalEvent,
} from "../fix-generated/erc20/erc20"
import { Approval, Balance, Transfer } from "../fix-generated/schema"
import { loadBalance, loadCollection, loadUser, updateAllowance, updateBalance } from "./helpers"
import { fetchAllowance } from "./utils"

export function handleTransfer(event: TransferOperatorEvent): void {
  const collection = loadCollection(event.address)
  // who operate to send transaction
  let operator = loadUser(event.params.operator)
  let from = loadUser(event.params.from)
  let to = loadUser(event.params.to)

  // Bypass Mint event
  if (!event.params.from.equals(Address.zero())) {
    let fromBalance = loadBalance(collection, from)
    updateBalance(fromBalance, Address.fromString(collection.id), Address.fromString(from.id))
    updateAllowance(collection, from, operator)
  }

  // Bypass Burn event
  if (!event.params.to.equals(Address.zero())) {
    let toBalance = loadBalance(collection, to)
    updateBalance(toBalance, Address.fromString(collection.id), Address.fromString(to.id))
  }

  let entity = new Transfer(
    event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString()
  )
  entity.operator = operator.id
  entity.from = from.id
  entity.to = to.id

  entity.collection = loadCollection(event.address).id

  entity.value = event.params.value
  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleApproval(event: ApprovalEvent): void {
  let collection = loadCollection(event.address)
  let owner = loadUser(event.params.owner)
  let spender = loadUser(event.params.spender)

  const id = collection.id + "-" + owner.id + "-" + spender.id

  let entity = Approval.load(id)
  if (!entity) {
    entity = new Approval(id)
    entity.owner = owner.id
    entity.spender = spender.id
    entity.value = event.params.value
    entity.remaining_allowance = event.params.value
    entity.collection = collection.id
  }

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash
  entity.remaining_allowance = fetchAllowance(Address.fromString(collection.id), Address.fromString(owner.id), Address.fromString(spender.id))

  entity.save()
}
