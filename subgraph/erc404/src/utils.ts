/* eslint-disable prefer-const */
import { Address, BigInt, Bytes, ipfs, json, JSONValue, Value } from '@graphprotocol/graph-ts'
import { erc404 } from '../fix-generated/erc404/erc404'

export function fetchName(address: Address): string {
    let contract = erc404.bind(address)
    let nameResult = contract.try_name()
    if (!nameResult.reverted) {
        return nameResult.value
    }
    return 'unknown'
}

export function fetchSymbol(address: Address): string {
    let contract = erc404.bind(address)
    let symbolResult = contract.try_symbol()
    if (!symbolResult.reverted) {
        return symbolResult.value
    }
    return 'unknown'
}

export function fetchTotalSupply(address: Address): BigInt {
    let contract = erc404.bind(address)
    let totalSupplyResult = contract.try_totalSupply()
    if (!totalSupplyResult.reverted) {
        return totalSupplyResult.value
    }
    return BigInt.fromI32(0)
}

export function fetchApprovedForAll(address: Address, owner: Address, spender: Address): bool {
    let contract = erc404.bind(address)
    let ret = contract.try_isApprovedForAll(owner, spender)
    if (!ret.reverted) {
        return ret.value
    }
    return false
}

export function fetchAllowance(address: Address, owner: Address, spender: Address): BigInt {
    let contract = erc404.bind(address)
    let allowance = contract.try_allowance(owner, spender)
    if (!allowance.reverted) {
        return allowance.value
    }
    return BigInt.fromI32(0)
}

export function fetchBalanceOf(address: Address, account: Address): BigInt {
    let contract = erc404.bind(address)
    let balanceResult = contract.try_balanceOf1(account)
    if (!balanceResult.reverted) {
        return balanceResult.value
    }
    return BigInt.fromI32(-1)
}

// Define the class for the return object
class TokenBalance {
    tokenId: BigInt;
    balance: BigInt;

    constructor(tokenId: BigInt, balance: BigInt) {
        this.tokenId = tokenId;
        this.balance = balance;
    }
}

export function fetchBalanceOfBatch(address: Address, account: Address): Array<TokenBalance> | null {
    let contract = erc404.bind(address)
    let tokenIdsRes = contract.try_ids()
    assert(!tokenIdsRes.reverted, "tokenIds is null")
    let tokenIds = tokenIdsRes.value

    let accounts = new Array<Address>(tokenIds.length).fill(account);
    let balanceResult = contract.try_balanceOfBatch(accounts, tokenIds)
    if (!balanceResult.reverted) {
        let result: TokenBalance[] = [];
        for (let i = 0; i < tokenIds.length; i++) {
            let tokenBalance = new TokenBalance(tokenIds[i], balanceResult.value[i]);
            result.push(tokenBalance);
        }
        return result;
    }
    return null
}
