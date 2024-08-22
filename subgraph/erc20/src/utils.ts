/* eslint-disable prefer-const */
import { Address, BigInt } from '@graphprotocol/graph-ts'
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

export function fetchBalanceOf(address: Address, user_address: Address): BigInt {
    let contract = erc20.bind(address)
    let balanceResult = contract.try_balanceOf(user_address)
    if (!balanceResult.reverted) {
        return balanceResult.value
    }
    return BigInt.fromI32(0)
}

export function fetchAllowance(address: Address, owner: Address, spender: Address): BigInt {
    let contract = erc20.bind(address)
    let allowanceResult = contract.try_allowance(owner, spender)
    if (!allowanceResult.reverted) {
        return allowanceResult.value
    }
    return BigInt.fromI32(0)
}

export function fetchTotalSupply(address: Address): BigInt {
    let contract = erc20.bind(address)
    let totalSupplyResult = contract.try_totalSupply()
    if (!totalSupplyResult.reverted) {
        return totalSupplyResult.value
    }
    return BigInt.fromI32(0)
}

