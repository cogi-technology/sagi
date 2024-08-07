// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {ERC721Holder} from "@openzeppelin/contracts/token/ERC721/utils/ERC721Holder.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract TokenERC721 is ERC721, Ownable, ERC721Holder {
    string private _customBaseURI;
    uint256 private _tokenIdCounter;

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

    function mint(address to) external onlyOwner {
        super._safeMint(to, _tokenIdCounter++);
    }
}
