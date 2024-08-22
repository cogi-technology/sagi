import { Address, BigDecimal, BigInt } from "@graphprotocol/graph-ts"
import { Collection, MetadataAttribute, NFT, User } from "../fix-generated/schema"
import { fetchMetadata, fetchName, fetchSymbol } from "./utils"

export const ZERO_ADDRESS = Address.zero()
export const ZERO_BI = BigInt.fromI32(0)
export const ONE_BI = BigInt.fromI32(1)
export const ZERO_BD = BigDecimal.fromString('0')

export function loadCollection(address: Address): Collection {
    let collection = Collection.load(address.toHex())
    if (collection == null) {
        collection = new Collection(address.toHex())
        collection.name = fetchName(address)
        collection.symbol = fetchSymbol(address)
        collection.save()
    }
    return collection
}

export function loadUser(address: Address): User {
    const _address = address.toHex()
    let user = User.load(_address)
    if (user == null) {
        user = new User(_address)
        user.numberTokens = ZERO_BI
        user.save()
    }
    return user
}

export function loadToken(address: Address, tokenId: BigInt, cid: string = ""): NFT {
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
    // check metadata exists
    assert(metadata != null, 'fetchMetadata failed')

    let metadataType = MetadataAttribute.load(cid + '-type')
    let metadataGrade = MetadataAttribute.load(cid + '-grade')

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