import {
  Approval as ApprovalEvent,
  ApprovalForAll as ApprovalForAllEvent,
  Initialized as InitializedEvent,
  OwnershipTransferred as OwnershipTransferredEvent,
  Paused as PausedEvent,
  RoleAdminChanged as RoleAdminChangedEvent,
  RoleGranted as RoleGrantedEvent,
  RoleRevoked as RoleRevokedEvent,
  Transfer as TransferEvent,
  TransferBatch as TransferBatchEvent,
  TransferSingle as TransferSingleEvent,
  URI as URIEvent,
  Unpaused as UnpausedEvent,
} from "../generated/erc404/erc404"
import {
  Erc404Approval,
  Erc404ApprovalForAll,
  Erc404OwnershipTransferred,
  Erc404Transfer,
  Erc404TransferBatch,
  Erc404TransferSingle,
  Erc404URI,
} from "../generated/schema"

export function handleApproval(event: ApprovalEvent): void {
  let entity = new Erc404Approval(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.owner = event.params.owner
  entity.spender = event.params.spender
  entity.value = event.params.value

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleApprovalForAll(event: ApprovalForAllEvent): void {
  let entity = new Erc404ApprovalForAll(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.account = event.params.account
  entity.operator = event.params.operator
  entity.approved = event.params.approved

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleOwnershipTransferred(
  event: OwnershipTransferredEvent,
): void {
  let entity = new Erc404OwnershipTransferred(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.previousOwner = event.params.previousOwner
  entity.newOwner = event.params.newOwner

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleTransfer(event: TransferEvent): void {
  let entity = new Erc404Transfer(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.from = event.params.from
  entity.to = event.params.to
  entity.value = event.params.value

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleTransferBatch(event: TransferBatchEvent): void {
  let entity = new Erc404TransferBatch(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.operator = event.params.operator
  entity.from = event.params.from
  entity.to = event.params.to
  entity.ids = event.params.ids
  entity.values = event.params.values

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleTransferSingle(event: TransferSingleEvent): void {
  let entity = new Erc404TransferSingle(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.operator = event.params.operator
  entity.from = event.params.from
  entity.to = event.params.to
  entity.erc404_id = event.params.id
  entity.value = event.params.value

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleURI(event: URIEvent): void {
  let entity = new Erc404URI(event.transaction.hash.concatI32(event.logIndex.toI32()))
  entity.value = event.params.value
  entity.erc404_id = event.params.id

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}