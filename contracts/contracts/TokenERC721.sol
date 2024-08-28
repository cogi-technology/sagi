// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {ERC721} from "./common/ERC721.sol";
import {ERC721URIStorage} from "./common/ERC721URIStorage.sol";
import {ERC721Pausable} from "./common/ERC721Pausable.sol";
import {ERC721Burnable} from "./common/ERC721Burnable.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {EIP712} from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {SafeERC20} from "./common/SafeERC20.sol";
import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {IERC20} from "./interfaces/IERC20.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/**
 * @title ERC721 Token
 */

contract TokenERC721 is
    EIP712,
    ERC721URIStorage,
    ERC721Burnable,
    ERC721Pausable,
    ReentrancyGuard,
    Ownable,
    AccessControl
{
    using SafeERC20 for IERC20;
    uint256 private _tokenIdIncr;
    mapping(string => uint8) public cids;
    string public __baseURI;
    mapping(uint256 => uint8) public locks;
    // mapping(address => uint256) public nonces;
    mapping(uint256 => uint256) public lastTransfer;
    uint256 public transferLock;
    IERC20 public currency;
    address public liquidityProviderAddress;
    mapping(address => uint8) public proxyRegistryAddress;

    event onAwardItems(address[] recipients, string[] cids, uint256[] tokenIds);
    event onAwardItem(address recipient, string cid, uint256 tokenId);
    event onTransfer(address from, address to, uint256 tokenId);
    event onBurn(uint256 tokenId);
    event onLock(uint256 tokenId);
    event onUnlock(uint256 tokenId);
    event onAwardItemWithFee(
        address recipient,
        string cid,
        uint256 tokenId,
        uint256 fee
    );

    error ERC721AccountTemporarilyUnavailable();
    error ERC721UnauthorizedMinter();

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant FREEZE_ROLE = keccak256("FREEZE_ROLE");

    constructor(
        address owner_,
        string memory name_,
        string memory symbol_,
        string memory baseURI_,
        address currency_
    ) ERC721(name_, symbol_) Ownable(owner_) EIP712("ERC721_EIP712", "1.0.0") {
        _grantRole(DEFAULT_ADMIN_ROLE, owner_);
        _grantRole(MINTER_ROLE, owner_);
        _grantRole(PAUSER_ROLE, owner_);

        __baseURI = baseURI_;
        _tokenIdIncr = 0;
        transferLock = 0;
        currency = IERC20(currency_);
    }

    function _baseURI() internal view virtual override returns (string memory) {
        return __baseURI;
    }

    function _burn(
        uint256 tokenId
    ) internal override(ERC721, ERC721URIStorage) {
        string memory _cid = _getCID(tokenId);
        super._burn(tokenId);
        if (bytes(_cid).length != 0) {
            cids[_cid] = 0;
        }
        emit onBurn(tokenId);
    }

    function force_burns(uint256[] memory tokenIds) public virtual {
        require(
            hasRole(DEFAULT_ADMIN_ROLE, _msgSender()),
            "ERC721: must have Admin role to burn"
        );
        for (uint256 i = 0; i < tokenIds.length; i++) {
            _burn(tokenIds[i]);
            emit onBurn(tokenIds[i]);
        }
    }

    function _isTransferable(uint256 tokenId) internal view returns (bool) {
        bool ret = lastTransfer[tokenId] <= blockHeight();
        return ret;
    }

    modifier nonUnfreeze(address from) {
        if (hasRole(FREEZE_ROLE, from)) {
            revert ERC721AccountTemporarilyUnavailable();
        }
        _;
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 tokenId
    ) internal virtual override(ERC721, ERC721Pausable) nonUnfreeze(from) {
        require(
            locks[tokenId] != 1 && _isTransferable(tokenId),
            "ERC721: was locking"
        );
        super._beforeTokenTransfer(from, to, tokenId);
        lastTransfer[tokenId] = blockHeight();
        emit onTransfer(from, to, tokenId);
    }

    function _awardItem(
        address recipient,
        string memory cid
    ) internal nonUnfreeze(recipient) returns (uint256 tokenId) {
        require(cids[cid] != 1, "_awardItem: cid invalid");
        cids[cid] = 1;
        uint256 newTokenId = _tokenIdIncr++;
        _mint(recipient, newTokenId);
        _setTokenURI(newTokenId, cid);
        return newTokenId;
    }

    // function _awardItemBySignatureHash(
    //     address account,
    //     string memory cid,
    //     uint256 deadline
    // ) internal view returns (bytes32) {
    //     return
    //         _hashTypedDataV4(
    //             keccak256(
    //                 abi.encode(
    //                     keccak256(
    //                         "AwardItemBySignature(address account,string cid,uint256 nonce,uint256 deadline)"
    //                     ),
    //                     account,
    //                     keccak256(bytes(cid)),
    //                     nonces[account],
    //                     deadline
    //                 )
    //             )
    //         );
    // }

    // function _awardItemsBySignatureHash(
    //     address account,
    //     string[] memory _cids,
    //     uint256 deadline
    // ) internal view returns (bytes32) {
    //     string memory cids_secure = "";
    //     for (uint256 i = 0; i < _cids.length; i++) {
    //         cids_secure = _append(cids_secure, _cids[i]);
    //     }
    //     string memory secure = string(
    //         abi.encodePacked(keccak256(bytes(cids_secure)))
    //     );
    //     return
    //         _hashTypedDataV4(
    //             keccak256(
    //                 abi.encode(
    //                     keccak256(
    //                         "AwardItemsBySignature(address account,string cids_secure,uint256 nonce,uint256 deadline)"
    //                     ),
    //                     account,
    //                     keccak256(bytes(secure)),
    //                     nonces[account],
    //                     deadline
    //                 )
    //             )
    //         );
    // }

    // function _awardItemsBySignatureToHash(
    //     address account,
    //     address[] memory recipients,
    //     string[] memory _cids,
    //     uint256 deadline
    // ) internal view returns (bytes32) {
    //     string memory cids_secure = "";
    //     string memory recipients_secure = "";
    //     for (uint256 i = 0; i < _cids.length; i++) {
    //         cids_secure = _append(cids_secure, _cids[i]);
    //         recipients_secure = _append(
    //             recipients_secure,
    //             to_string(recipients[i])
    //         );
    //     }

    //     string memory cid_secure = string(
    //         abi.encodePacked(keccak256(bytes(cids_secure)))
    //     );
    //     string memory recipient_secure = string(
    //         abi.encodePacked(keccak256(bytes(recipients_secure)))
    //     );
    //     return
    //         _hashTypedDataV4(
    //             keccak256(
    //                 abi.encode(
    //                     keccak256(
    //                         "AwardItemsBySignatureTo(address account,string recipients_secure,string cids_secure,uint256 nonce,uint256 deadline)"
    //                     ),
    //                     account,
    //                     keccak256(bytes(recipient_secure)),
    //                     keccak256(bytes(cid_secure)),
    //                     nonces[account],
    //                     deadline
    //                 )
    //             )
    //         );
    // }

    function to_string(address a) public pure returns (string memory) {
        return Strings.toHexString(a);
    }

    function _append(
        string memory a,
        string memory b
    ) internal pure returns (string memory) {
        return string(abi.encodePacked(a, b));
    }

    function _verify(
        bytes32 digest,
        bytes memory signature
    ) internal view returns (bool) {
        return hasRole(MINTER_ROLE, ECDSA.recover(digest, signature));
    }

    function transferableAt(uint256 tokenId) internal view returns (uint256) {
        return lastTransfer[tokenId] + transferLock;
    }

    //onwer view
    function setBaseURI(string memory baseURI) external onlyOwner {
        __baseURI = baseURI;
    }

    function setLpAddress(address _lpAddress) external onlyOwner {
        liquidityProviderAddress = _lpAddress;
    }

    function _update(
        address to,
        uint256 tokenId,
        address auth
    ) internal virtual override(ERC721, ERC721Pausable) returns (address) {
        return super._update(to, tokenId, auth);
    }

    // function setOpenBoxTime(uint256 _time) external onlyOwner {
    //     openBoxTime = _time;
    // }

    function setProxyRegistryAddress(
        address _proxyRegistryAddress
    ) external onlyOwner {
        proxyRegistryAddress[_proxyRegistryAddress] = 1;
    }

    function removeProxyRegistryAddress(
        address _proxyRegistryAddress
    ) external onlyOwner {
        require(
            (proxyRegistryAddress[_proxyRegistryAddress] == 1),
            "ERC721: proxyAdress input invalid"
        );
        delete proxyRegistryAddress[_proxyRegistryAddress];
    }

    //admin view
    function lpWithdraw(uint256 _amount) external {
        require(_amount > 0, "charge amount should be more than 0");
        require(
            _msgSender() == liquidityProviderAddress,
            "you are not liquidity Provider"
        );
        uint256 _balance = currency.balanceOf(address(this));
        require(_balance >= _amount, "withdraw balance not enough");
        currency.safeTransfer(_msgSender(), _amount);
    }

    function awardItems(
        address[] memory _recipients,
        string[] memory _cids
    ) external {
        require(
            hasRole(MINTER_ROLE, _msgSender()),
            "ERC721: must have minter role to awardItem"
        );
        uint256[] memory tokenIds = new uint256[](_recipients.length);
        for (uint256 i = 0; i < _recipients.length; i++) {
            tokenIds[i] = _awardItem(_recipients[i], _cids[i]);
            emit onAwardItem(_recipients[i], _cids[i], tokenIds[i]);
        }
    }

    function awardItem(
        address recipient,
        string memory cid
    ) external returns (uint256) {
        require(
            hasRole(MINTER_ROLE, _msgSender()),
            "ERC721: must have minter role to awardItem"
        );
        uint256 tokenId = _awardItem(recipient, cid);
        emit onAwardItem(recipient, cid, tokenId);
        return tokenId;
    }

    function lock(uint256 tokenId) external {
        require(
            hasRole(MINTER_ROLE, _msgSender()),
            "ERC721: must have minter role to lock"
        );
        locks[tokenId] = 1;
        emit onLock(tokenId);
    }

    function unlock(uint256 tokenId) external {
        require(
            hasRole(MINTER_ROLE, _msgSender()),
            "ERC721: must have minter role to unlock"
        );
        require((locks[tokenId] == 1), "ERC721: unlock input invalid");
        delete locks[tokenId];
        emit onUnlock(tokenId);
    }

    function pause() external virtual {
        require(
            hasRole(PAUSER_ROLE, _msgSender()),
            "ERC721: must have pauser role to pause"
        );
        _pause();
    }

    function unpause() external virtual {
        require(
            hasRole(PAUSER_ROLE, _msgSender()),
            "ERC721: must have pauser role to unpause"
        );
        _unpause();
    }

    function freeze(address account) external {
        if (!hasRole(MINTER_ROLE, _msgSender())) {
            revert ERC721UnauthorizedMinter();
        }

        grantRole(FREEZE_ROLE, account);
    }

    function unfreeze(address account) external {
        if (!hasRole(MINTER_ROLE, _msgSender())) {
            revert ERC721UnauthorizedMinter();
        }

        revokeRole(FREEZE_ROLE, account);
    }

    /////////////////////////MINTER_ROLE/////////////////////////

    function setTransferLock(uint256 _transferLock) external {
        require(
            hasRole(DEFAULT_ADMIN_ROLE, _msgSender()),
            "ERC721: must have admin role to setTransferLock"
        );
        transferLock = _transferLock;
    }

    //anon view
    function getCID(uint256 tokenId) external view returns (string memory) {
        return _getCID(tokenId);
    }

    // function awardItemBySignature(
    //     string memory cid,
    //     uint256 deadline,
    //     bytes calldata signature
    // ) external nonReentrant {
    //     address account = _msgSender();
    //     require(
    //         timestamp() <= deadline,
    //         "awardItemBySignature: Expired transaction"
    //     );
    //     require(
    //         _verify(
    //             _awardItemBySignatureHash(account, cid, deadline),
    //             signature
    //         ),
    //         "awardItemBySignature: Invalid signature"
    //     );
    //     uint256 tokenId = _awardItem(account, cid);
    //     nonces[account]++;
    //     emit onAwardItem(account, cid, tokenId);
    // }

    // function awardItemsBySignature(
    //     string[] memory _cids,
    //     uint256 deadline,
    //     bytes calldata signature
    // ) external nonReentrant {
    //     address account = _msgSender();
    //     require(
    //         timestamp() <= deadline,
    //         "awardItemsBySignature: Expired transaction"
    //     );
    //     require(
    //         _verify(
    //             _awardItemsBySignatureHash(account, _cids, deadline),
    //             signature
    //         ),
    //         "awardItemsBySignature: Invalid signature"
    //     );
    //     for (uint256 i = 0; i < _cids.length; i++) {
    //         string memory cid = _cids[i];
    //         uint256 tokenId = _awardItem(account, cid);
    //         nonces[account]++;
    //         emit onAwardItem(account, cid, tokenId);
    //     }
    // }

    // function awardItemBySignatureTo(
    //     address recipient,
    //     string memory cid,
    //     uint256 deadline,
    //     bytes calldata signature
    // ) external nonReentrant returns (uint256) {
    //     require(
    //         timestamp() <= deadline,
    //         "awardItemBySignature: Expired transaction"
    //     );
    //     require(
    //         _verify(
    //             _awardItemBySignatureHash(recipient, cid, deadline),
    //             signature
    //         ),
    //         "awardItemBySignature: Invalid signature"
    //     );
    //     uint256 tokenId = _awardItem(recipient, cid);
    //     nonces[recipient]++;
    //     emit onAwardItem(recipient, cid, tokenId);
    //     return tokenId;
    // }

    // function awardItemsBySignatureTo(
    //     address[] memory _recipients,
    //     string[] memory _cids,
    //     uint256 deadline,
    //     bytes calldata signature
    // ) external nonReentrant {
    //     address account = _msgSender();
    //     require(
    //         timestamp() <= deadline,
    //         "awardItemsBySignature: Expired transaction"
    //     );
    //     require(
    //         _verify(
    //             _awardItemsBySignatureToHash(
    //                 account,
    //                 _recipients,
    //                 _cids,
    //                 deadline
    //             ),
    //             signature
    //         ),
    //         "awardItemsBySignatureTo: Invalid signature"
    //     );
    //     for (uint256 i = 0; i < _cids.length; i++) {
    //         uint256 tokenId = _awardItem(_recipients[i], _cids[i]);
    //         nonces[account]++;
    //         emit onAwardItem(_recipients[i], _cids[i], tokenId);
    //     }
    // }

    // function awardItemWithFeeBySignature(
    //     string memory cid,
    //     uint256 deadline,
    //     uint256 fee,
    //     bytes calldata signature
    // ) external nonReentrant {
    //     address account = _msgSender();
    //     require(
    //         timestamp() <= deadline,
    //         "awardItemWithFeeBySignature: Expired transaction"
    //     );
    //     require(
    //         _verify(
    //             _awardItemWithFeeBySignatureHash(account, cid, deadline, fee),
    //             signature
    //         ),
    //         "awardItemWithFeeBySignature: Invalid signature"
    //     );
    //     require(currency.balanceOf(account) >= fee, "Balance not enough");
    //     require(
    //         currency.allowance(account, address(this)) >= fee,
    //         "fee exceeds allowance"
    //     );
    //     currency.safeTransferFrom(account, liquidityProviderAddress, fee);
    //     uint256 tokenId = _awardItem(account, cid);
    //     nonces[account]++;
    //     emit onAwardItemWithFee(account, cid, tokenId, fee);
    // }

    function tokenIdsOf(
        address account,
        uint256 maxToken
    ) external view returns (uint256[] memory) {
        uint256 retCount = 0;
        uint256[] memory tempTokenIds = new uint256[](maxToken);
        uint256 j = 0;
        for (uint256 i = 0; i <= _tokenIdIncr; i++) {
            if (isOwnerOf(account, i)) {
                tempTokenIds[j] = i;
                j++;
                retCount++;
            }
            if (maxToken <= retCount) break;
        }
        uint256[] memory tokenIds = new uint256[](retCount);
        for (uint256 i = 0; i < retCount; i++) {
            tokenIds[i] = tempTokenIds[i];
        }
        return tokenIds;
    }

    function tokenURI(
        uint256 tokenId
    ) public view override(ERC721, ERC721URIStorage) returns (string memory) {
        return super.tokenURI(tokenId);
    }

    function supportsInterface(
        bytes4 interfaceId
    )
        public
        view
        virtual
        override(ERC721, ERC721URIStorage, AccessControl)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }

    /**
     * Override isApprovedForAll to whitelist market contract.
     */
    function isApprovedForAll(
        address owner,
        address operator
    ) public view override returns (bool) {
        if (proxyRegistryAddress[operator] == 1) {
            return true;
        }

        return super.isApprovedForAll(owner, operator);
    }

    function timestamp() public view returns (uint256) {
        return block.timestamp;
    }

    function blockHeight() public view returns (uint256) {
        return block.number;
    }

    function version() external view virtual returns (uint256) {
        return 202301011;
    }

    /*
        t.Approval
        t.ApprovalForAll
        t.DEFAULT_ADMIN_ROLE
        t.OwnershipTransferred
        t.Paused
        t.RoleAdminChanged
        t.RoleGranted
        t.RoleRevoked
        t.Transfer
        t.Unpaused
        t.abi
        t.address
        t.allEvents
        t.approve
        t.awardItem
        t.balanceOf
        t.burn
        t.constructor
        t.contract
        t.getApproved
        t.getPastEvents
        t.getRoleAdmin
        t.getRoleMember
        t.getRoleMemberCount
        t.grantRole
        t.hasRole
        t.initialize
        t.isApprovedForAll
        t.methods
        t.name
        t.owner
        t.ownerOf
        t.paused
        t.renounceOwnership
        t.renounceRole
        t.revokeRole
        t.safeMint
        t.safeTransferFrom
        t.send
        t.sendTransaction
        t.setApprovalForAll
        t.supportsInterface
        t.symbol
        t.tokenByIndex
        t.tokenOfOwnerByIndex 
        t.tokenURI
        t.totalSupply
        t.transactionHash
        t.transferFrom
        t.transferOwnership
        t.freeze
        t.unfreeze
    */
}
