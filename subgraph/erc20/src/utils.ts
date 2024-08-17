/* eslint-disable prefer-const */
import { Address } from '@graphprotocol/graph-ts'
import { erc20 } from '../fix-generated/erc20/erc20'

export function fetchName(address: Address): string {
    let contract = erc20.bind(address)
    let nameResult = contract.try_name()
    if (!nameResult.reverted) {
        return nameResult.value
    }
    return 'unknown'
}

export function fetchSymbol(address: Address): string {
    let contract = erc20.bind(address)
    let symbolResult = contract.try_symbol()
    if (!symbolResult.reverted) {
        return symbolResult.value
    }
    return 'unknown'
}