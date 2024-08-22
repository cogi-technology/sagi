// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {ERC721} from "./common/ERC721.sol";
import {ERC721Holder} from "@openzeppelin/contracts/token/ERC721/utils/ERC721Holder.sol";
import {ERC721URIStorage} from "./common/ERC721URIStorage.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract TokenERC721 is ERC721, Ownable, ERC721Holder, ERC721URIStorage {
    string private _customBaseURI;
    uint256 private _tokenIdCounter;
    mapping(string => uint8) public cids;

    event AwardItem(
        address indexed recipient,
        uint256 indexed tokenId,
        string cid
    );
    event Burn(uint256 indexed tokenId);

    constructor(
        address owner_,
        string memory name_,
        string memory symbol_,
        string memory baseURI_
    ) ERC721(name_, symbol_) Ownable(owner_) {
        _customBaseURI = baseURI_;
        _tokenIdCounter = 0;
    }

    function _baseURI() internal view virtual override returns (string memory) {
        return _customBaseURI;
    }

    function _awardItem(
        address recipient,
        string memory cid
    ) internal returns (uint256 tokenId) {
        require(cids[cid] != 1, "_awardItem: cid invalid");
        cids[cid] = 1;
        uint256 newTokenId = _tokenIdCounter++;
        super._safeMint(recipient, newTokenId);

        _setTokenURI(newTokenId, cid);

        emit AwardItem(recipient, newTokenId, cid);
        return newTokenId;
    }

    function awardItem(
        address recipient,
        string memory cid
    ) external onlyOwner {
        _awardItem(recipient, cid);
    }

    function awardItems(
        address[] memory _recipients,
        string[] memory _cids
    ) external onlyOwner {
        uint256[] memory tokenIds = new uint256[](_recipients.length);
        for (uint256 i = 0; i < _recipients.length; i++) {
            tokenIds[i] = _awardItem(_recipients[i], _cids[i]);
        }
    }

    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override(ERC721, ERC721URIStorage) returns (bool) {
        return super.supportsInterface(interfaceId);
    }

    function tokenURI(
        uint256 tokenId
    )
        public
        view
        virtual
        override(ERC721, ERC721URIStorage)
        returns (string memory)
    {
        return super.tokenURI(tokenId);
    }

    function _burn(
        uint256 tokenId
    ) internal override(ERC721, ERC721URIStorage) {
        string memory _cid = _getCID(tokenId);
        require(cids[_cid] == 1, "_awardItem: cid invalid");

        super._burn(tokenId);
        if (bytes(_cid).length != 0) {
            cids[_cid] = 0;
        }
        emit Burn(tokenId);
    }

    function force_burns(uint256[] memory tokenIds) public virtual onlyOwner {
        for (uint256 i = 0; i < tokenIds.length; i++) {
            _burn(tokenIds[i]);
        }
    }
}
