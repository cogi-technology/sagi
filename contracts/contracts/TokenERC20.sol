// SPDX-License-Identifier: MIT
pragma solidity ^0.8.2;

import {ERC20} from "./common/ERC20.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC20Pausable} from "./common/ERC20Pausable.sol";
import {ERC20Permit} from "./common/ERC20Permit.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title ERC20 Token
 * @author ZION Inc
 */

contract TokenERC20 is
    ERC20,
    Ownable,
    ERC20Pausable,
    ERC20Permit,
    AccessControl
{
    error ERC20ExceededCap(uint256 increasedSupply, uint256 cap);
    error ERC20ProxyInputInvalid();
    error ERC20UnauthorizedMinter();
    error ERC20UnauthorizedAdmin();
    error ERC20AccountTemporarilyUnavailable();

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant FREEZE_ROLE = keccak256("FREEZE_ROLE");
    // uint256 private _maxSupply; // 0 is unlimited
    // uint8 private _decimals;
    mapping(address => uint8) public proxyRegistryAddress;

    constructor(
        address owner_,
        string memory name_,
        string memory symbol_,
        uint256 initialSupply_
    ) ERC20(name_, symbol_) Ownable(owner_) ERC20Permit(name_) {
        _grantRole(DEFAULT_ADMIN_ROLE, owner_);
        _grantRole(MINTER_ROLE, owner_);

        _mint(owner_, initialSupply_);

        // _decimals = decimals_;
        // _maxSupply = maxSupply_;
    }

    // function decimals() public view override returns (uint8) {
    //     return _decimals;
    // }

    ////////////////// ADMIN /////////////////
    function setProxyRegistryAddress(
        address _proxyRegistryAddress
    ) public onlyOwner {
        proxyRegistryAddress[_proxyRegistryAddress] = 1;
    }

    function removeProxyRegistryAddress(
        address _proxyRegistryAddress
    ) public onlyOwner {
        if (proxyRegistryAddress[_proxyRegistryAddress] != 1) {
            revert ERC20ProxyInputInvalid();
        }
        delete proxyRegistryAddress[_proxyRegistryAddress];
    }

    // function mint(address account, uint256 amount) external {
    //     if (!hasRole(MINTER_ROLE, _msgSender())) {
    //         revert ERC20UnauthorizedMinter();
    //     }

    //     _mint(account, amount);
    // }

    // function burnFrom(address account, uint256 amount) external {
    //     if (!hasRole(MINTER_ROLE, _msgSender())) {
    //         revert ERC20UnauthorizedMinter();
    //     }

    //     _burn(account, amount);
    // }

    function addMinter(address account) external {
        if (!hasRole(DEFAULT_ADMIN_ROLE, _msgSender())) {
            revert ERC20UnauthorizedAdmin();
        }

        grantRole(MINTER_ROLE, account);
    }

    function removeMinter(address account) external {
        if (!hasRole(DEFAULT_ADMIN_ROLE, _msgSender())) {
            revert ERC20UnauthorizedAdmin();
        }

        revokeRole(MINTER_ROLE, account);
    }

    function pause() external {
        if (!hasRole(DEFAULT_ADMIN_ROLE, _msgSender())) {
            revert ERC20UnauthorizedAdmin();
        }

        _pause();
    }

    function unpause() external {
        if (!hasRole(DEFAULT_ADMIN_ROLE, _msgSender())) {
            revert ERC20UnauthorizedAdmin();
        }

        _unpause();
    }

    ////////////////// MINTER /////////////////
    function freeze(address account) external {
        if (!hasRole(MINTER_ROLE, _msgSender())) {
            revert ERC20UnauthorizedMinter();
        }

        grantRole(FREEZE_ROLE, account);
    }

    function unfreeze(address account) external {
        if (!hasRole(MINTER_ROLE, _msgSender())) {
            revert ERC20UnauthorizedMinter();
        }

        revokeRole(FREEZE_ROLE, account);
    }

    function _update(
        address from,
        address to,
        uint256 value
    ) internal override(ERC20, ERC20Pausable) {
        if (hasRole(FREEZE_ROLE, from)) {
            revert ERC20AccountTemporarilyUnavailable();
        }

        super._update(from, to, value);

        // if (from == address(0) && _maxSupply > 0) {
        //     uint256 supply = totalSupply();
        //     if (supply > _maxSupply) {
        //         revert ERC20ExceededCap(supply, _maxSupply);
        //     }
        // }
    }

    ////////////////// ANON /////////////////
    function burn(uint256 amount) external {
        _burn(_msgSender(), amount);
    }

    function allowance(
        address owner,
        address spender
    ) public view override returns (uint256) {
        if (proxyRegistryAddress[spender] == 1) return type(uint256).max;
        return super.allowance(owner, spender);
    }

    // function maxSupply() external view returns (uint256) {
    //     if (_maxSupply > 0) {
    //         return _maxSupply;
    //     } else {
    //         return totalSupply();
    //     }
    // }

    function version() external view virtual returns (uint256) {
        return 202406171430;
    }
}
