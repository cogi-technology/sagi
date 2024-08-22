import { Address, log } from "@graphprotocol/graph-ts"
import {
  AwardItem as AwardItemEvent,
  Burn as BurnEvent,
  Transfer as TransferEvent,
  Approval as ApprovalEvent,
  ApprovalForAll as ApprovalForAllEvent,
} from "../fix-generated/erc721/erc721"
import {
  ApprovalForAll,
  AwardItem,
  Burn,
  Transfer,
  User
} from "../fix-generated/schema"
import { fetchMetadata, fetchName, fetchSymbol } from "./utils"
import { IMetadata } from "./types"
import { Approval } from "../fix-generated/schema"
import { loadCollection, loadToken, loadUser, ONE_BI, ZERO_ADDRESS } from "./helpers"

export function handleAwardItem(event: AwardItemEvent): void {
  let collection = loadCollection(event.address)
  let recipient = loadUser(event.params.recipient)
  log.info('handleAwardItem {} {} {}', [event.address.toHex(), recipient.id, event.params.tokenId.toString()])

  let nft = loadToken(Address.fromString(collection.id), event.params.tokenId, event.params.cid);
  nft.owner = recipient.id
  nft.updatedAt = event.block.timestamp
  nft.save()

  recipient.numberTokens = recipient.numberTokens.plus(ONE_BI)
  recipient.save()

  let entity = new AwardItem(event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString())
  entity.recipient = event.params.recipient
  entity.cid = event.params.cid
  entity.tokenId = event.params.tokenId

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.collection = collection.id;
  entity.nft = nft.id;

  entity.save()
}

export function handleBurn(event: BurnEvent): void {
  let collection = loadCollection(event.address)
  let nft = loadToken(Address.fromString(collection.id), event.params.tokenId);
  let old_owner = User.load(nft.owner)!;
  let zero_address = loadUser(ZERO_ADDRESS);

  nft.owner = zero_address.id
  nft.updatedAt = event.block.timestamp
  nft.save()

  zero_address.numberTokens = zero_address.numberTokens.plus(ONE_BI)
  zero_address.save()

  old_owner.numberTokens = old_owner.numberTokens.minus(ONE_BI)
  old_owner.save()

  let entity = new Burn(event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString())

  entity.tokenId = event.params.tokenId

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.collection = collection.id;
  entity.nft = nft.id;

  entity.save()
}

export function handleTransfer(event: TransferEvent): void {
  //bypass Mint event or Burn event
  if (event.params.from == Address.zero() || event.params.to == Address.zero()) {
    return
  }

  let collection = loadCollection(event.address)
  let sender = loadUser(event.params.from)
  let recipient = loadUser(event.params.to)
  let nft = loadToken(Address.fromString(collection.id), event.params.tokenId);
  nft.owner = recipient.id
  nft.updatedAt = event.block.timestamp
  nft.save()

  sender.numberTokens = sender.numberTokens.minus(ONE_BI)
  sender.save()
  recipient.numberTokens = recipient.numberTokens.plus(ONE_BI)
  recipient.save()

  let entity = new Transfer(event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString())

  entity.from = event.params.from
  entity.to = event.params.to
  entity.tokenId = event.params.tokenId

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.collection = collection.id;
  entity.nft = nft.id;

  entity.save()
}

export function handleApproval(event: ApprovalEvent): void {
  let collection = loadCollection(event.address)
  let owner = loadUser(event.params.owner)
  let spender = loadUser(event.params.approved)

  let entity = new Approval(event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString())
  entity.owner = owner.id
  entity.approved = spender.id
  entity.tokenId = event.params.tokenId

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.collection = collection.id;

  entity.save()
}

export function handleApprovalForAll(event: ApprovalForAllEvent): void {
  let collection = loadCollection(event.address)
  let owner = loadUser(event.params.owner)
  let operator = loadUser(event.params.operator)

  let entity = new ApprovalForAll(event.transaction.hash.toHex() + "-" + event.logIndex.toI32().toString())
  entity.owner = owner.id
  entity.operator = operator.id
  entity.approved = event.params.approved

  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.collection = collection.id;

  entity.save()
}