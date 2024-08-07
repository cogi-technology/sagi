// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import { IERC20 } from  "./IERC20.sol";
import { IERC1155 } from  "./IERC1155.sol";

interface IERC404 is IERC20, IERC1155 {
    /**
    @notice Get the balance of an account's tokens.
    @param _owner  The address of the token holder
    @param _id     ID of the token
    @return        The _owner's balance of the token type requested
    */
    function erc1155BalanceOf(
        address _owner,
        uint256 _id
    ) external view returns (uint256);

    /**
    * @dev Returns the value of tokens owned by `account`.
    */
    function erc20BalanceOf(address account) external view returns (uint256);

    /**
     * @notice Function to check if address is transfer exempt
     */
    function erc1155TransferExempt(
        address target_
    ) external view returns (bool);
}