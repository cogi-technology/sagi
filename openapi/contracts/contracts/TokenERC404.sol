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

    event Mint(address indexed account, uint256 amount);
    event Burn(address indexed account, uint256 amount);

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant FREEZE_ROLE = keccak256("FREEZE_ROLE");

    uint256[] private _ids;
    uint256 private _units;
    uint8 private _decimals;

    constructor(
        address owner_,
        string memory name_,
        string memory symbol_,
        uint8 decimals_,
        uint256 units_,
        uint256[] memory ids_,
        string memory uri_
    ) Ownable(owner_) ReentrancyGuard() Pausable() {
        __ERC404_init(name_, symbol_, units_, ids_, uri_);
        // _erc20Mint(owner_, initialSupply_);
        _erc1155SetTransferExempt(owner_, true);
        _grantRole(DEFAULT_ADMIN_ROLE, owner_);
        _grantRole(MINTER_ROLE, owner_);

        _decimals = decimals_;
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

    function _mint(address account, uint256 amount) internal {
        uint256 b = balanceOf(account);
        _erc20Mint(account, amount);
        uint256 a = balanceOf(account);
        if (!erc1155TransferExempt(account)) {
            uint256 need = (a / _units) - (b / _units);
            if (need > 0) {
                _erc1155Mints(account, _ids, need);
            }
        }
        emit Mint(account, amount);
    }

    function _burn(address account, uint256 amount) internal {
        uint256 b = balanceOf(account);
        _erc20Burn(account, amount);
        uint256 a = balanceOf(account);
        if (!erc1155TransferExempt(account)) {
            uint256 need = (b / _units) - (a / _units);
            if (need > 0) {
                _erc1155Burns(account, _ids, need);
            }
        }
        emit Burn(account, amount);
    }

    function mint(address account, uint256 amount) external {
        if (!hasRole(MINTER_ROLE, _msgSender())) {
            revert ERC404UnauthorizedMinter();
        }
        _mint(account, amount);
    }

    function burnFrom(address account, uint256 amount) external {
        if (!hasRole(MINTER_ROLE, _msgSender())) {
            revert ERC404UnauthorizedMinter();
        }
        _burn(account, amount);
    }

    function burn(uint256 amount) external {
        _burn(_msgSender(), amount);
    }

    function addTransferExempt(
        address target_
    ) external onlyOwner returns (bool) {
        return _erc1155SetTransferExempt(target_, true);
    }

    function decimals() public view override returns (uint8) {
        return _decimals;
    }

    function ids() public view returns (uint256[] memory) {
        return _ids;
    }

    function units() public view returns (uint256) {
        return _units;
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
        address from_,
        address to_,
        uint256[] memory ids_,
        uint256[] memory values_,
        bytes memory data_
    ) public virtual override nonReentrant {
        return
            super.erc1155SafeBatchTransferFrom(
                from_,
                to_,
                ids_,
                values_,
                data_
            );
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
