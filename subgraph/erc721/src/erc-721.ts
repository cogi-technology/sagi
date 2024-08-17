import { Address, BigDecimal, BigInt, Bytes, dataSource, json, log } from "@graphprotocol/graph-ts"
import {
  AwardItem as AwardItemEvent,
  Burn as BurnEvent,
  Transfer as TransferEvent
} from "../generated/erc721/erc721"
import {
  AwardItem,
  Burn,
  Collection,
  Metadata,
  MetadataAttribute,
  NFT,
  Transfer,
  User
} from "../generated/schema"
import { fetchMetadata, fetchName, fetchSymbol } from "./utils"
import { IMetadata } from "./types"

const ZERO_ADDRESS = Address.zero()
const ZERO_BI = BigInt.fromI32(0)
const ONE_BI = BigInt.fromI32(1)
const ZERO_BD = BigDecimal.fromString('0')

function loadCollection(address: Address): Collection {
  let collection = Collection.load(address.toHex())
  if (collection == null) {
    collection = new Collection(address.toHex())
    collection.name = fetchName(address)
    collection.symbol = fetchSymbol(address)
    collection.save()
  }
  return collection
}

function loadUser(address: Address): User {
  const _address = address.toHex()
  let user = User.load(_address)
  if (user == null) {
    user = new User(_address)
    user.numberTokens = ZERO_BI
    user.save()
  }
  return user
}

function loadToken(address: Address, tokenId: BigInt, cid: string = ""): NFT {
  const _tid = address.toHex() + '-' + tokenId.toString()

  // check nft exists 
  if (cid == "") {
    let token = NFT.load(_tid)
    assert(token != null, `loadToken failed ${address.toHex()} ${tokenId} ${cid}`)
    return token as NFT
  }

  //new token
  const collection = loadCollection(address)
  let token = new NFT(_tid)
  let metadata = fetchMetadata(cid) // //IMetadata.default(cid) //
  let metadataType = MetadataAttribute.load(cid + '-type')
  let metadataGrade = MetadataAttribute.load(cid + '-grade')

  // check metadata exists
  assert(metadata != null, 'fetchMetadata failed')
  // assert(metadataType != null, 'load MetadataType failed')
  // assert(metadataGrade != null, 'load MetadataGrade failed')

  // save token entity
  token.tokenId = tokenId
  token.collection = collection.id
  token.metadata = metadata!.id
  token.metadataName = metadata!.name.toLowerCase()
  token.metadataType = metadataType ? metadataType.value : null
  token.metadataGrade = metadataGrade ? <i32>parseFloat(metadataGrade.value as string) : -1
  token.updatedAt = ZERO_BI
  token.owner = ZERO_ADDRESS.toHex()
  token.save()

  return token
}

export function handleMetadata(content: Bytes): void {
  let cid = dataSource.stringParam()

  let tokenMetadata = new Metadata(cid)
  log.info("[+] loadToken::handleMetadata: {}", [dataSource.stringParam()])

  const value = json.fromBytes(content).toObject()
  if (value) {
    const image = value.get('image')
    const name = value.get('name')
    const description = value.get('description')
    const externalURL = value.get('external_url')

    let imetadata = IMetadata.from(cid, value)

    if (name && image && description && externalURL) {
      tokenMetadata.name = name.toString()
      tokenMetadata.image = image.toString()
      tokenMetadata.externalURL = externalURL.toString()
      tokenMetadata.description = description.toString()
    }

    tokenMetadata.save()
  }
}

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

  let entity = new AwardItem(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
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

  nft.owner = ZERO_ADDRESS.toHex()
  nft.updatedAt = event.block.timestamp
  nft.save()

  old_owner.numberTokens = old_owner.numberTokens.minus(ONE_BI)
  old_owner.save()

  let entity = new Burn(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
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

  let entity = new Transfer(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )

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

