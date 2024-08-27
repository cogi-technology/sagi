import { Address } from "@graphprotocol/graph-ts"
import {
  Approval as ApprovalEvent,
  ApprovalForAll as ApprovalForAllEvent,
  Burn as BurnEvent,
  ERC1155SetTransferExempt as ERC1155SetTransferExemptEvent,
  Mint as MintEvent,
  TransferOperator as TransferOperatorEvent,
  TransferBatch as TransferBatchEvent,
  TransferSingle as TransferSingleEvent,
} from "../fix-generated/erc404/erc404"
import {
  Approval,
  ApprovalForAll,
  Burn,
  Mint,
  Transfer,
  TransferBatch,
  TransferSingle,
} from "../fix-generated/schema"
import { loadCollection, loadErc1155BalanceOfBatch, loadErc20Balance, loadUser, updateAllowance, updateErc1155BatchBalance, updateErc20Balance } from "./helpers"
import { fetchAllowance } from "./utils"

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
    entity.remaining_allowance = event.params.value
    entity.collection = collection.id
  }
  entity.value = event.params.value

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash
  entity.remaining_allowance = event.params.value //fetchAllowance(Address.fromString(collection.id), Address.fromString(owner.id), Address.fromString(spender.id))

  entity.save()
}

export function handleApprovalForAll(event: ApprovalForAllEvent): void {
  const collection = loadCollection(event.address)
  const account = loadUser(event.params.account)
  const operator = loadUser(event.params.operator)

  const id = collection.id + "-" + account.id + "-" + operator.id;

  let entity = ApprovalForAll.load(id)

  if (!entity) {
    entity = new ApprovalForAll(id)
    entity.account = account.id
    entity.operator = operator.id
    entity.collection = collection.id
  }
  entity.approved = event.params.approved
  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleBurn(event: BurnEvent): void {
  const collection = loadCollection(event.address)
  let account = loadUser(event.params.account)
  let erc20Balance = loadErc20Balance(collection, account)
  let erc1155BatchBalance = loadErc1155BalanceOfBatch(collection, account)

  // update erc404 balance of account
  updateErc20Balance(erc20Balance, Address.fromString(collection.id), Address.fromString(account.id))
  updateErc1155BatchBalance(erc1155BatchBalance, Address.fromString(collection.id), Address.fromString(account.id))

  let entity = new Burn(
    event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString()
  )
  entity.account = account.id
  entity.amount = event.params.amount

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash
  entity.collection = collection.id

  entity.save()
}

export function handleERC1155SetTransferExempt(
  event: ERC1155SetTransferExemptEvent
): void {
  let user = loadUser(event.params.account)
  user.isErc1155TransferExempt = event.params.exempt
  user.save()
}

export function handleMint(event: MintEvent): void {
  const collection = loadCollection(event.address)
  let account = loadUser(event.params.account)
  let erc20Balance = loadErc20Balance(collection, account)
  let erc1155BatchBalance = loadErc1155BalanceOfBatch(collection, account)
  updateErc20Balance(erc20Balance, Address.fromString(collection.id), Address.fromString(account.id))
  updateErc1155BatchBalance(erc1155BatchBalance, Address.fromString(collection.id), Address.fromString(account.id))

  let entity = new Mint(
    event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString()
  )
  entity.account = account.id
  entity.amount = event.params.amount

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash
  entity.collection = collection.id

  entity.save()
}

export function handleTransfer(event: TransferOperatorEvent): void {
  //bypass Mint event or Burn event
  if (event.params.from == Address.zero() || event.params.to == Address.zero()) {
    return
  }

  const collection = loadCollection(event.address)
  let operator = loadUser(event.params.operator)
  let from = loadUser(event.params.from)
  let to = loadUser(event.params.to)

  let erc20FromBalance = loadErc20Balance(collection, from)
  updateErc20Balance(erc20FromBalance, Address.fromString(collection.id), Address.fromString(from.id))
  let erc20ToBalance = loadErc20Balance(collection, to)
  updateErc20Balance(erc20ToBalance, Address.fromString(collection.id), Address.fromString(to.id))

  // event.receipt.
  updateAllowance(collection.id, from.id, operator.id, event.params.value)

  let entity = new Transfer(
    event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString()
  )
  entity.operator = operator.id
  entity.from = from.id
  entity.to = to.id
  entity.value = event.params.value

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash
  entity.collection = collection.id

  entity.save()
}

export function handleTransferBatch(event: TransferBatchEvent): void {
  const collection = loadCollection(event.address)
  let operator = loadUser(event.params.operator)
  let from = loadUser(event.params.from)
  let to = loadUser(event.params.to)

  let erc1155FromBatchBalance = loadErc1155BalanceOfBatch(collection, from)
  updateErc1155BatchBalance(erc1155FromBatchBalance, Address.fromString(collection.id), Address.fromString(from.id))
  let erc1155ToBatchBalance = loadErc1155BalanceOfBatch(collection, to)
  updateErc1155BatchBalance(erc1155ToBatchBalance, Address.fromString(collection.id), Address.fromString(to.id))

  let entity = new TransferBatch(
    event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString()
  )
  entity.operator = operator.id
  entity.from = from.id
  entity.to = to.id
  entity.ids = event.params.ids
  entity.values = event.params.values

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash
  entity.collection = collection.id

  entity.save()
}

export function handleTransferSingle(event: TransferSingleEvent): void {
  const collection = loadCollection(event.address)
  let operator = loadUser(event.params.operator)
  let from = loadUser(event.params.from)
  let to = loadUser(event.params.to)

  let erc1155FromBatchBalance = loadErc1155BalanceOfBatch(collection, from)
  updateErc1155BatchBalance(erc1155FromBatchBalance, Address.fromString(collection.id), Address.fromString(from.id))
  let erc1155ToBatchBalance = loadErc1155BalanceOfBatch(collection, to)
  updateErc1155BatchBalance(erc1155ToBatchBalance, Address.fromString(collection.id), Address.fromString(to.id))

  let entity = new TransferSingle(
    event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString()
  )
  entity.operator = operator.id
  entity.from = from.id
  entity.to = to.id
  entity.erc1155_id = event.params.id
  entity.value = event.params.value

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash
  entity.collection = collection.id

  entity.save()
}