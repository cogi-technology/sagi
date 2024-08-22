import { Address, BigInt } from "@graphprotocol/graph-ts"
import { Approval, Collection, ERC1155Balance, ERC1155BatchBalance, ERC20Balance, User } from "../fix-generated/schema"
import { fetchAllowance, fetchBalanceOf, fetchBalanceOfBatch, fetchName, fetchSymbol, fetchTotalSupply } from "./utils"

export function loadCollection(address: Address): Collection {
    let collection = Collection.load(address.toHex())

    if (collection == null) {
        collection = new Collection(address.toHex())
        collection.name = fetchName(address)
        collection.symbol = fetchSymbol(address)
    }
    let totalSupply = fetchTotalSupply(address)
    collection.totalSupply = totalSupply
    collection.save()

    return collection
}

export function loadUser(address: Address): User {
    const _address = address.toHex()
    let user = User.load(_address)
    if (user == null) {
        user = new User(_address)
        user.isErc1155TransferExempt = false
        user.save()
    }
    return user
}

export function updateErc20Balance(entity: ERC20Balance, collection_address: Address, user_address: Address): void {
    let newBalance = fetchBalanceOf(collection_address, user_address)
    entity.balance = newBalance
    entity.save()
}

export function updateErc1155BatchBalance(entity: ERC1155BatchBalance, collection_address: Address, user_address: Address): void {
    let balanceOfBatch = fetchBalanceOfBatch(collection_address, user_address)
    assert(balanceOfBatch != null, "balanceOfBatch is null")

    let erc1155Balance_ids: string[] = [];
    for (let i = 0; i < balanceOfBatch!.length; i++) {
        let e = ERC1155Balance.load(entity.id + "-" + balanceOfBatch![i].tokenId.toString());
        if (e == null) {
            e = new ERC1155Balance(entity.id + "-" + balanceOfBatch![i].tokenId.toString());
            e.tokenId = balanceOfBatch![i].tokenId
            e.balance = BigInt.fromI32(0)
        }
        e.balance = balanceOfBatch![i].balance;
        e.save();
        erc1155Balance_ids.push(e.id);
    }

    entity.batchBalance = erc1155Balance_ids;
    entity.save()
}

export function updateAllowance(collection: Collection, owner: User, spender: User): void {
    if (owner.id == spender.id) {
        return
    }

    let allowanceEntity = Approval.load(collection.id + "-" + owner.id + "-" + spender.id)
    assert(allowanceEntity !== null, "allowanceEntity is null")

    allowanceEntity!.remaining_allowance = fetchAllowance(Address.fromString(collection.id), Address.fromString(owner.id), Address.fromString(spender.id))
    allowanceEntity!.save()
}

export function loadErc20Balance(collection: Collection, user: User): ERC20Balance {
    const id = collection.id + "-" + user.id;
    let entity = ERC20Balance.load(id)

    if (entity == null) {
        entity = new ERC20Balance(id)
        entity.collection = collection.id
        entity.account = user.id
        entity.balance = BigInt.fromI32(0)
        entity.save()
    }
    return entity
}

export function loadErc1155BalanceOfBatch(collection: Collection, user: User): ERC1155BatchBalance {
    const id = collection.id + "-" + user.id;
    let entity = ERC1155BatchBalance.load(id)
    if (entity == null) {
        entity = new ERC1155BatchBalance(id)
        entity.collection = collection.id
        entity.account = user.id
        entity.batchBalance = []

        entity.save()
    }
    return entity;
}