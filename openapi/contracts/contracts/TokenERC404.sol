//SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {ERC404} from "./common/ERC404.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

contract TokenERC404 is
    Initializable,
    Context,
    Ownable,
    ERC404,
    ReentrancyGuard,
    AccessControl,
    Pausable
{
    error ERC404UnauthorizedMinter();
    error ERC404UnauthorizedAdmin();
    error ERC404AccountTemporarilyUnavailable();

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant FREEZE_ROLE = keccak256("FREEZE_ROLE");

    uint256[] private _ids;
    uint256 private _units;
    uint8 private _decimals;

    constructor(
        address owner_,
        string memory name_,
        string memory symbol_,
        uint256 initialSupply_,
        uint256 units_,
        uint256[] memory ids_,
        string memory uri_
    ) Ownable(owner_) ReentrancyGuard() Pausable() {
        __ERC404_init(name_, symbol_, units_, ids_, uri_);
        _erc20Mint(owner_, initialSupply_);
        _erc1155SetTransferExempt(owner_, true);
        _grantRole(DEFAULT_ADMIN_ROLE, owner_);
        _grantRole(MINTER_ROLE, owner_);

        _decimals = 18;
        _ids = ids_;
        _units = units_;
    }

    function _erc20Update(
        address from,
        address to,
        uint256 value
    ) internal override whenNotPaused {
        if (hasRole(FREEZE_ROLE, from)) {
            revert ERC404AccountTemporarilyUnavailable();
        }

        super._erc20Update(from, to, value);
    }

    function addTransferExempt(
        address target_
    ) external onlyOwner returns (bool) {
        return _erc1155SetTransferExempt(target_, true);
    }

    function decimals() public view override returns (uint8) {
        return _decimals;
    }

    function removeTransferExempt(
        address target_
    ) external onlyOwner returns (bool) {
        return _erc1155SetTransferExempt(target_, false);
    }

    function freeze(address account) external {
        if (!hasRole(MINTER_ROLE, _msgSender())) {
            revert ERC404UnauthorizedMinter();
        }

        grantRole(FREEZE_ROLE, account);
    }

    function unfreeze(address account) external {
        if (!hasRole(MINTER_ROLE, _msgSender())) {
            revert ERC404UnauthorizedMinter();
        }

        revokeRole(FREEZE_ROLE, account);
    }

    function addMinter(address account) external {
        if (!hasRole(DEFAULT_ADMIN_ROLE, _msgSender())) {
            revert ERC404UnauthorizedAdmin();
        }

        grantRole(MINTER_ROLE, account);
    }

    function removeMinter(address account) external {
        if (!hasRole(DEFAULT_ADMIN_ROLE, _msgSender())) {
            revert ERC404UnauthorizedAdmin();
        }

        revokeRole(MINTER_ROLE, account);
    }

    function pause() external {
        if (!hasRole(DEFAULT_ADMIN_ROLE, _msgSender())) {
            revert ERC404UnauthorizedAdmin();
        }

        _pause();
    }

    function unpause() external {
        if (!hasRole(DEFAULT_ADMIN_ROLE, _msgSender())) {
            revert ERC404UnauthorizedAdmin();
        }

        _unpause();
    }

    function transfer(
        address to,
        uint256 value
    ) public virtual override nonReentrant returns (bool) {
        return super.transfer(to, value);
    }

    function transferFrom(
        address from,
        address to,
        uint256 value
    ) public virtual override nonReentrant returns (bool) {
        return super.transferFrom(from, to, value);
    }

    function erc115SafeTransferFrom(
        address from,
        address to,
        uint256 id,
        uint256 value,
        bytes memory data
    ) public virtual override nonReentrant {
        return super.erc115SafeTransferFrom(from, to, id, value, data);
    }

    function erc1155SafeBatchTransferFrom(
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory values,
        bytes memory data
    ) public virtual override nonReentrant {
        return super.erc1155SafeBatchTransferFrom(from, to, ids, values, data);
    }

    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override(AccessControl, ERC404) returns (bool) {
        return super.supportsInterface(interfaceId);
    }

    function version() external view virtual returns (uint256) {
        return 202406121315;
    }
}
