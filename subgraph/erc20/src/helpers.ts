import { Address, BigInt, log } from "@graphprotocol/graph-ts"
import { Approval, Balance, Collection, User } from "../fix-generated/schema"
import { fetchAllowance, fetchBalanceOf, fetchName, fetchSymbol, fetchTotalSupply } from "./utils"


export function loadCollection(address: Address): Collection {
    let collection = Collection.load(address.toHex())
    if (collection == null) {
        collection = new Collection(address.toHex())
        collection.name = fetchName(address)
        collection.symbol = fetchSymbol(address)
        collection.totalSupply = fetchTotalSupply(address)
        collection.save()
    }
    return collection
}

export function loadUser(address: Address): User {
    const _address = address.toHex()
    let user = User.load(_address)
    if (user == null) {
        user = new User(_address)
        user.save()
    }
    return user
}

export function loadBalance(collection: Collection, user: User): Balance {
    const id = collection.id + "-" + user.id;
    let entity = Balance.load(id)

    if (entity == null) {
        entity = new Balance(id)
        entity.collection = collection.id
        entity.account = user.id
        entity.balance = BigInt.fromI32(0)
        entity.save()
    }
    return entity
}

export function updateAllowance(collection: string, owner: string, spender: string, value: BigInt): void {
    if (owner == spender) {
        return
    }

    let allowanceEntity = Approval.load(collection + "-" + owner + "-" + spender)
    assert(allowanceEntity !== null, "allowanceEntity is null")

    allowanceEntity!.remaining_allowance = allowanceEntity!.remaining_allowance.minus(value);
    allowanceEntity!.save()
}

export function updateBalance(entity: Balance, collection_address: Address, user_address: Address): void {
    let newBalance = fetchBalanceOf(collection_address, user_address)
    entity.balance = newBalance
    entity.save()
}