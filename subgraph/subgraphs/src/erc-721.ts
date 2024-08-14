import {
  Approval as ApprovalEvent,
  ApprovalForAll as ApprovalForAllEvent,
  AwardItem as AwardItemEvent,
  Burn as BurnEvent,
  OwnershipTransferred as OwnershipTransferredEvent,
  SetTokenCID as SetTokenCIDEvent,
  Transfer as TransferEvent,
} from "../generated/erc721/erc721"
import {
  Erc721Approval,
  Erc721ApprovalForAll,
  Erc721AwardItem,
  Erc721Burn,
  Erc721OwnershipTransferred,
  Erc721SetTokenCID,
  Erc721Transfer,
} from "../generated/schema"

export function handleApproval(event: ApprovalEvent): void {
  let entity = new Erc721Approval(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.owner = event.params.owner
  entity.approved = event.params.approved
  entity.tokenId = event.params.tokenId

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleApprovalForAll(event: ApprovalForAllEvent): void {
  let entity = new Erc721ApprovalForAll(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.owner = event.params.owner
  entity.operator = event.params.operator
  entity.approved = event.params.approved

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleAwardItem(event: AwardItemEvent): void {
  let entity = new Erc721AwardItem(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.recipient = event.params.recipient
  entity.cid = event.params.cid
  entity.tokenId = event.params.tokenId

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleBurn(event: BurnEvent): void {
  let entity = new Erc721Burn(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.tokenId = event.params.tokenId

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleOwnershipTransferred(
  event: OwnershipTransferredEvent,
): void {
  let entity = new Erc721OwnershipTransferred(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.previousOwner = event.params.previousOwner
  entity.newOwner = event.params.newOwner

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleSetTokenCID(event: SetTokenCIDEvent): void {
  let entity = new Erc721SetTokenCID(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.tokenId = event.params.tokenId
  entity.cid = event.params.cid

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

export function handleTransfer(event: TransferEvent): void {
  let entity = new Erc721Transfer(
    event.transaction.hash.concatI32(event.logIndex.toI32()),
  )
  entity.from = event.params.from
  entity.to = event.params.to
  entity.tokenId = event.params.tokenId

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}
