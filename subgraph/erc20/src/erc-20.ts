import { Address } from "@graphprotocol/graph-ts"
import {
  Transfer as TransferEvent,
} from "../fix-generated/erc20/erc20"
import { Transfer, User, Collection } from "../fix-generated/schema"
import { fetchName, fetchSymbol } from "./utils"

export function handleTransfer(event: TransferEvent): void {
  let entity = new Transfer(
    event.transaction.hash.concatI32(event.logIndex.toI32())
  )
  entity.from = loadUser(event.params.from).id
  entity.to = loadUser(event.params.to).id

  entity.collection = loadCollection(event.address).id

  entity.value = event.params.value
  entity.blockNumber = event.block.number
  entity.blockTimestamp = event.block.timestamp
  entity.transactionHash = event.transaction.hash

  entity.save()
}

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
    user.address = address
    user.save()
  }
  return user
}