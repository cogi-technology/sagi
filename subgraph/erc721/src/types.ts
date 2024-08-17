/* eslint-disable prefer-const */
import { Address, BigDecimal, BigInt, JSONValue } from '@graphprotocol/graph-ts'
import { ethereum } from '@graphprotocol/graph-ts/chain/ethereum'

export class IBidOrder {
    buyer: Address
    price: BigInt
}

export class IMetadata {
    id: string
    name: string
    description: string
    image: string
    attributes: IMetadataAttribute[]
    constructor(
        id: string,
        name: string,
        description: string,
        image: string,
        attributes: IMetadataAttribute[],
    ) {
        this.id = id
        this.name = name
        this.description = description
        this.image = image
        this.attributes = attributes
    }

    static default(cid: string): IMetadata {
        return new IMetadata(
            cid,
            "",
            "",
            "",
            [IMetadataAttribute.default(), IMetadataAttribute.default(), {
                "display_type": "",
                "trait_type": "Type",
                "value": "Mystic Medal"
            }, {
                "display_type": "",
                "trait_type": "Type",
                "value": "Supreme Box"
            }],
        )
    }

    static from(cid: string, value: JSONValue): IMetadata {
        let obj = value.toObject();

        let attributes: IMetadataAttribute[] = []
        if (obj.get('attributes') != null) {
            let _attributes = (obj.get('attributes') as JSONValue).toArray()
            for (let i = 0; i < _attributes.length; i++) {
                if (_attributes[i] == null) continue
                attributes[i] = new IMetadataAttribute()
                attributes[i].display_type = ''
                attributes[i].trait_type = ''
                attributes[i].value = ''
                let _o = _attributes[i].toObject()
                if (_o.get('display_type') != null) {
                    let v = _o.get('display_type') as JSONValue
                    if (!v.isNull()) {
                        attributes[i].display_type = changetype<string>(v.data as u32)
                    }
                }
                if (_o.get('trait_type') != null) {
                    let v = _o.get('trait_type') as JSONValue
                    if (!v.isNull()) {
                        attributes[i].trait_type = changetype<string>(v.data as u32)
                    }
                }
                if (_o.get('value') != null) {
                    let v = _o.get('value') as JSONValue
                    if (!v.isNull()) {
                        attributes[i].value = changetype<string>(v.data as u32)
                    }
                }
            }
        }
        assert(obj.get('name') != null, `Invalid metadata.name ${cid}`)
        assert(obj.get('description') != null, `Invalid metadata.description ${cid}`)
        assert(obj.get('image') != null, `Invalid metadata.image ${cid}`)

        let v = obj.get('name') as JSONValue
        let name = changetype<string>(v.data as u32)
        v = obj.get('description') as JSONValue
        let description = changetype<string>(v.data as u32)
        v = obj.get('image') as JSONValue
        let image = changetype<string>(v.data as u32)
        return new IMetadata(cid, name, description, image, attributes)
    }
}

export class IMetadataAttribute {
    display_type: string
    trait_type: string
    value: string

    static default(): IMetadataAttribute {
        return {
            display_type: "",
            trait_type: "",
            value: "",
        }
    }
}

export class ITransactionEvent {
    transaction: ethereum.Transaction
    block: ethereum.Block
    senderAddress: Address
    recipientAddress: Address
}
