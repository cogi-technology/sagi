/* eslint-disable prefer-const */
import { Address, ipfs, json, JSONValue, Value } from '@graphprotocol/graph-ts'
import { erc721 } from '../fix-generated/erc721/erc721'
import { IMetadata, IMetadataAttribute } from './types'
import { Metadata, MetadataAttribute } from '../generated/schema'

export function fetchName(address: Address): string {
    let contract = erc721.bind(address)
    let nameResult = contract.try_name()
    if (!nameResult.reverted) {
        return nameResult.value
    }
    return 'unknown'
}

export function fetchSymbol(address: Address): string {
    let contract = erc721.bind(address)
    let symbolResult = contract.try_symbol()
    if (!symbolResult.reverted) {
        return symbolResult.value
    }
    return 'unknown'
}

export function fetchMetadata(cid: string): Metadata | null {
    let bytes = ipfs.cat(cid)
    if (!bytes) {
        return null
    }
    let value = json.fromBytes(bytes)
    let metadata = processIpfsMetadata(value, Value.fromString(cid))

    return metadata
}

export function processIpfsMetadata(value: JSONValue, userData: Value): Metadata {
    let cid = userData.toString()
    let imetadata = IMetadata.from(cid, value) //IMetadata.default(cid) // 
    let metadata = Metadata.load(cid);

    if (metadata) {
        return metadata
    }

    metadata = new Metadata(cid)

    for (let i = 0; i < imetadata.attributes.length; i++) {
        let metadataAttribute = imetadata.attributes[i].trait_type === '' ? MetadataAttribute.load(cid + '-' + i.toString()) : MetadataAttribute.load(cid + '-' + (imetadata.attributes[i].trait_type as string).toLowerCase())
        if (metadataAttribute == null) {
            // new metadataAttribute entity
            let postfix = i.toString();
            if (imetadata.attributes[i].trait_type != '') {
                postfix = (imetadata.attributes[i].trait_type as string).toLowerCase()
            }
            metadataAttribute = new MetadataAttribute(cid + '-' + postfix)

            if (imetadata.attributes[i].display_type != null) {
                metadataAttribute.display_type = imetadata.attributes[i].display_type as string
            }
            if (imetadata.attributes[i].trait_type != null) {
                metadataAttribute.trait_type = imetadata.attributes[i].trait_type as string
            }
            if (imetadata.attributes[i].value != null) {
                metadataAttribute.value = imetadata.attributes[i].value as string
            }

            metadataAttribute.metadata = metadata.id
            metadataAttribute.save()
        }
    }

    metadata.name = imetadata.name
    metadata.description = imetadata.description
    metadata.image = imetadata.image
    metadata.save()

    return metadata
}