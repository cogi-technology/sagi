// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IERC404} from "../interfaces/IERC404.sol";
import {IERC20} from "../interfaces/IERC20.sol";
import {IERC1155} from "../interfaces/IERC1155.sol";
import {IERC404Metadata} from "../interfaces/IERC404Metadata.sol";
import {IERC404Errors} from "../interfaces/IERC404Errors.sol";
import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC1155Receiver} from "@openzeppelin/contracts/token/ERC1155/IERC1155Receiver.sol";
import {ERC165} from "@openzeppelin/contracts/utils/introspection/ERC165.sol";
import {Arrays} from "@openzeppelin/contracts/utils/Arrays.sol";

abstract contract ERC404 is
    Context,
    ERC165,
    IERC404,
    IERC404Metadata,
    IERC404Errors
{
    using Arrays for uint256[];
    using Arrays for address[];

    event ERC1155SetTransferExempt(address indexed account, bool exempt);

    struct ERC1155Data {
        uint256 id;
        uint256 value;
    }

    /// @custom:storage-location erc7201:openzeppelin.storage.ERC404
    struct ERC404Storage {
        //erc20
        mapping(address account => uint256) _balances;
        mapping(address account => mapping(address spender => uint256)) _allowances;
        uint256 _totalSupply;
        string _name;
        string _symbol;
        //erc1155
        uint256[] _ids;
        mapping(uint256 id => mapping(address account => uint256)) _erc1155Balances;
        mapping(address account => mapping(address operator => bool)) _operatorApprovals;
        // Used as the URI for all token types by relying on ID substitution, e.g. https://token-cdn-domain/{id}.json
        string _uri;
        //erc404
        uint256 _units;
        mapping(address => bool) _erc1155TransferExempt;
    }

    // keccak256(abi.encode(uint256(keccak256("openzeppelin.storage.ERC404")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant ERC404StorageLocation =
        0x4a9688b9ce80577b3b0de47295ed3c32cb849942b4221c0b478ab1f30a130100;

    function _getERC404Storage()
        private
        pure
        returns (ERC404Storage storage $)
    {
        assembly {
            $.slot := ERC404StorageLocation
        }
    }

    /**
     * @dev Sets the values for {name} and {symbol}.
     *
     * All two of these values are immutable: they can only be set once during
     * construction.
     */
    function __ERC404_init(
        string memory name_,
        string memory symbol_,
        uint256 units_,
        uint256[] memory ids_,
        string memory uri_
    ) internal {
        __ERC404_init_unchained(name_, symbol_, units_, ids_, uri_);
    }

    function __ERC404_init_unchained(
        string memory name_,
        string memory symbol_,
        uint256 units_,
        uint256[] memory ids_,
        string memory uri_
    ) internal {
        ERC404Storage storage $ = _getERC404Storage();
        $._name = name_;
        $._symbol = symbol_;
        $._units = units_;
        $._ids = ids_;
        $._uri = uri_;
    }

    /**
     * @dev Returns the name of the token.
     */
    function name() public view virtual returns (string memory) {
        ERC404Storage storage $ = _getERC404Storage();
        return $._name;
    }

    /**
     * @dev Returns the symbol of the token, usually a shorter version of the
     * name.
     */
    function symbol() public view virtual returns (string memory) {
        ERC404Storage storage $ = _getERC404Storage();
        return $._symbol;
    }

    /**
     * @dev Returns the number of decimals used to get its user representation.
     * For example, if `decimals` equals `2`, a balance of `505` tokens should
     * be displayed to a user as `5.05` (`505 / 10 ** 2`).
     *
     * Tokens usually opt for a value of 18, imitating the relationship between
     * Ether and Wei. This is the default value returned by this function, unless
     * it's overridden.
     *
     * NOTE: This information is only used for _display_ purposes: it in
     * no way affects any of the arithmetic of the contract, including
     * {IERC20-balanceOf} and {IERC20-transfer}.
     */
    function decimals() public view virtual returns (uint8) {
        return 18;
    }

    /**
     * @dev See {IERC1155MetadataURI-uri}.
     *
     * This implementation returns the same URI for *all* token types. It relies
     * on the token type ID substitution mechanism
     * https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
     *
     * Clients calling this function must replace the `\{id\}` substring with the
     * actual token type ID.
     */
    function uri(uint256 /* id */) public view virtual returns (string memory) {
        ERC404Storage storage $ = _getERC404Storage();
        return $._uri;
    }

    /**
     * @dev See {IERC20-totalSupply}.
     */
    function totalSupply() public view virtual returns (uint256) {
        ERC404Storage storage $ = _getERC404Storage();
        return $._totalSupply;
    }

    /**
     * @dev See {IERC20-balanceOf}.
     */
    function balanceOf(address account) public view virtual returns (uint256) {
        return erc20BalanceOf(account);
    }

    /**
     * @dev See {IERC20-balanceOf}.
     */
    function erc20BalanceOf(
        address account
    ) public view virtual returns (uint256) {
        ERC404Storage storage $ = _getERC404Storage();
        return $._balances[account];
    }

    /**
     * @dev See {IERC20-transfer}.
     *
     * Requirements:
     *
     * - `to` cannot be the zero address.
     * - the caller must have a balance of at least `value`.
     */
    function transfer(address to, uint256 value) public virtual returns (bool) {
        address owner = _msgSender();
        _transfer(owner, to, value);
        return true;
    }

    /**
     * @dev See {IERC20-allowance}.
     */
    function allowance(
        address owner,
        address spender
    ) public view virtual returns (uint256) {
        ERC404Storage storage $ = _getERC404Storage();
        return $._allowances[owner][spender];
    }

    /**
     * @dev See {IERC20-approve}.
     *
     * NOTE: If `value` is the maximum `uint256`, the allowance is not updated on
     * `transferFrom`. This is semantically equivalent to an infinite approval.
     *
     * Requirements:
     *
     * - `spender` cannot be the zero address.
     */
    function approve(
        address spender,
        uint256 value
    ) public virtual returns (bool) {
        address owner = _msgSender();
        _erc20Approve(owner, spender, value);
        return true;
    }

    /**
     * @dev See {IERC20-transferFrom}.
     *
     * Emits an {Approval} event indicating the updated allowance. This is not
     * required by the EIP. See the note at the beginning of {ERC20}.
     *
     * NOTE: Does not update the allowance if the current allowance
     * is the maximum `uint256`.
     *
     * Requirements:
     *
     * - `from` and `to` cannot be the zero address.
     * - `from` must have a balance of at least `value`.
     * - the caller must have allowance for ``from``'s tokens of at least
     * `value`.
     */
    function transferFrom(
        address from,
        address to,
        uint256 value
    ) public virtual returns (bool) {
        address spender = _msgSender();
        _spendAllowance(from, spender, value);
        _transfer(from, to, value);
        return true;
    }

    /**
     * @dev Moves a `value` amount of tokens from `from` to `to`.
     *
     * This internal function is equivalent to {transfer}, and can be used to
     * e.g. implement automatic token fees, slashing mechanisms, etc.
     *
     * Emits a {Transfer} event.
     *
     * NOTE: This function is not virtual, {_update} should be overridden instead.
     */
    function _erc20Transfer(address from, address to, uint256 value) internal {
        if (from == address(0)) {
            revert ERC20InvalidSender(address(0));
        }
        if (to == address(0)) {
            revert ERC20InvalidReceiver(address(0));
        }
        _erc20Update(from, to, value);
    }

    /**
     * @dev Transfers a `value` amount of tokens from `from` to `to`, or alternatively mints (or burns) if `from`
     * (or `to`) is the zero address. All customizations to transfers, mints, and burns should be done by overriding
     * this function.
     *
     * Emits a {Transfer} event.
     */
    function _erc20Update(
        address from,
        address to,
        uint256 value
    ) internal virtual {
        ERC404Storage storage $ = _getERC404Storage();
        address operator = _msgSender();

        if (from == address(0)) {
            // Overflow check required: The rest of the code assumes that totalSupply never overflows
            $._totalSupply += value;
        } else {
            uint256 fromBalance = $._balances[from];
            if (fromBalance < value) {
                revert ERC20InsufficientBalance(from, fromBalance, value);
            }
            unchecked {
                // Overflow not possible: value <= fromBalance <= totalSupply.
                $._balances[from] = fromBalance - value;
            }
        }

        if (to == address(0)) {
            unchecked {
                // Overflow not possible: value <= totalSupply or value <= fromBalance <= totalSupply.
                $._totalSupply -= value;
            }
        } else {
            unchecked {
                // Overflow not possible: balance + value is at most totalSupply, which we know fits into a uint256.
                $._balances[to] += value;
            }
        }

        emit Transfer(from, to, value);
        emit TransferOperator(operator, from, to, value);
    }

    /**
     * @dev Creates a `value` amount of tokens and assigns them to `account`, by transferring it from address(0).
     * Relies on the `_update` mechanism
     *
     * Emits a {Transfer} event with `from` set to the zero address.
     *
     * NOTE: This function is not virtual, {_update} should be overridden instead.
     */
    function _erc20Mint(address account, uint256 value) internal {
        if (account == address(0)) {
            revert ERC20InvalidReceiver(address(0));
        }
        _erc20Update(address(0), account, value);
    }

    /**
     * @dev Destroys a `value` amount of tokens from `account`, lowering the total supply.
     * Relies on the `_update` mechanism.
     *
     * Emits a {Transfer} event with `to` set to the zero address.
     *
     * NOTE: This function is not virtual, {_update} should be overridden instead
     */
    function _erc20Burn(address account, uint256 value) internal {
        if (account == address(0)) {
            revert ERC20InvalidSender(address(0));
        }
        _erc20Update(account, address(0), value);
    }

    /**
     * @dev Sets `value` as the allowance of `spender` over the `owner` s tokens.
     *
     * This internal function is equivalent to `approve`, and can be used to
     * e.g. set automatic allowances for certain subsystems, etc.
     *
     * Emits an {Approval} event.
     *
     * Requirements:
     *
     * - `owner` cannot be the zero address.
     * - `spender` cannot be the zero address.
     *
     * Overrides to this logic should be done to the variant with an additional `bool emitEvent` argument.
     */
    function _erc20Approve(
        address owner,
        address spender,
        uint256 value
    ) internal {
        _erc20Approve(owner, spender, value, true);
    }

    /**
     * @dev Variant of {_approve} with an optional flag to enable or disable the {Approval} event.
     *
     * By default (when calling {_approve}) the flag is set to true. On the other hand, approval changes made by
     * `_spendAllowance` during the `transferFrom` operation set the flag to false. This saves gas by not emitting any
     * `Approval` event during `transferFrom` operations.
     *
     * Anyone who wishes to continue emitting `Approval` events on the`transferFrom` operation can force the flag to
     * true using the following override:
     * ```
     * function _approve(address owner, address spender, uint256 value, bool) internal virtual override {
     *     super._approve(owner, spender, value, true);
     * }
     * ```
     *
     * Requirements are the same as {_approve}.
     */
    function _erc20Approve(
        address owner,
        address spender,
        uint256 value,
        bool emitEvent
    ) internal virtual {
        ERC404Storage storage $ = _getERC404Storage();
        if (owner == address(0)) {
            revert ERC20InvalidApprover(address(0));
        }
        if (spender == address(0)) {
            revert ERC20InvalidSpender(address(0));
        }
        $._allowances[owner][spender] = value;
        if (emitEvent) {
            emit Approval(owner, spender, value);
        }
    }

    /**
     * @dev Updates `owner` s allowance for `spender` based on spent `value`.
     *
     * Does not update the allowance value in case of infinite allowance.
     * Revert if not enough allowance is available.
     *
     * Does not emit an {Approval} event.
     */
    function _spendAllowance(
        address owner,
        address spender,
        uint256 value
    ) internal virtual {
        uint256 currentAllowance = allowance(owner, spender);
        if (currentAllowance != type(uint256).max) {
            if (currentAllowance < value) {
                revert ERC20InsufficientAllowance(
                    spender,
                    currentAllowance,
                    value
                );
            }
            unchecked {
                _erc20Approve(owner, spender, currentAllowance - value, false);
            }
        }
    }

    //ERC1155
    /**
     * @dev See {IERC1155-balanceOf}.
     */
    function balanceOf(
        address account,
        uint256 id
    ) public view virtual returns (uint256) {
        return erc1155BalanceOf(account, id);
    }

    /**
     * @dev See {IERC1155-balanceOfBatch}.
     *
     * Requirements:
     *
     * - `accounts` and `ids` must have the same length.
     */
    function balanceOfBatch(
        address[] memory accounts,
        uint256[] memory ids
    ) public view virtual returns (uint256[] memory) {
        return erc1155BalanceOfBatch(accounts, ids);
    }

    /**
     * @dev See {IERC1155-balanceOfBatch}.
     *
     * Requirements:
     *
     * - `accounts` and `ids` must have the same length.
     */
    function erc1155BalanceOfBatch(
        address[] memory accounts,
        uint256[] memory ids
    ) public view virtual returns (uint256[] memory) {
        if (accounts.length != ids.length) {
            revert ERC1155InvalidArrayLength(ids.length, accounts.length);
        }

        uint256[] memory batchBalances = new uint256[](accounts.length);

        for (uint256 i = 0; i < accounts.length; ++i) {
            batchBalances[i] = erc1155BalanceOf(
                accounts.unsafeMemoryAccess(i),
                ids.unsafeMemoryAccess(i)
            );
        }

        return batchBalances;
    }

    /**
     * @dev See {IERC1155-setApprovalForAll}.
     */
    function setApprovalForAll(address operator, bool approved) public virtual {
        _erc1155SetApprovalForAll(_msgSender(), operator, approved);
    }

    /**
     * @dev See {IERC1155-isApprovedForAll}.
     */
    function isApprovedForAll(
        address account,
        address operator
    ) public view virtual returns (bool) {
        ERC404Storage storage $ = _getERC404Storage();
        return $._operatorApprovals[account][operator];
    }

    /**
     * @dev See {IERC1155-safeTransferFrom}.
     */
    function safeTransferFrom(
        address from,
        address to,
        uint256 id,
        uint256 value,
        bytes memory data
    ) public virtual {
        erc115SafeTransferFrom(from, to, id, value, data);
    }

    /**
     * @dev See {IERC1155-safeTransferFrom}.
     */
    function erc115SafeTransferFrom(
        address from,
        address to,
        uint256 id,
        uint256 value,
        bytes memory data
    ) public virtual {
        ERC404Storage storage $ = _getERC404Storage();
        address sender = _msgSender();
        if (from != sender && !isApprovedForAll(from, sender)) {
            revert ERC1155MissingApprovalForAll(sender, from);
        }
        _erc1155SafeTransferFrom(from, to, id, value, data);
        _erc20Transfer(from, to, value * $._units);
    }

    /**
     * @dev See {IERC1155-safeBatchTransferFrom}.
     */
    function safeBatchTransferFrom(
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory values,
        bytes memory data
    ) public virtual {
        erc1155SafeBatchTransferFrom(from, to, ids, values, data);
    }

    /**
     * @dev See {IERC1155-safeBatchTransferFrom}.
     */
    function erc1155SafeBatchTransferFrom(
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory values,
        bytes memory data
    ) public virtual {
        ERC404Storage storage $ = _getERC404Storage();
        address sender = _msgSender();
        if (from != sender && !isApprovedForAll(from, sender)) {
            revert ERC1155MissingApprovalForAll(sender, from);
        }
        _erc1155SafeBatchTransferFrom(from, to, ids, values, data);
        uint256 total = 0;
        for (uint256 i = 0; i < values.length; i++) {
            total += values[i];
        }
        _erc20Transfer(from, to, total * $._units);
    }

    /**
     * @notice Initialization function to set pairs / etc, saving gas by avoiding mint / burn on unnecessary targets
     */
    function _erc1155SetTransferExempt(
        address target_,
        bool state_
    ) internal returns (bool) {
        if (target_ == address(0)) {
            revert ERC404InvalidExemption(target_);
        }
        ERC404Storage storage $ = _getERC404Storage();
        $._erc1155TransferExempt[target_] = state_;

        emit ERC1155SetTransferExempt(target_, state_);
        return true;
    }

    /**
     * @notice Function to check if address is transfer exempt
     */
    function erc1155TransferExempt(address target_) public view returns (bool) {
        ERC404Storage storage $ = _getERC404Storage();
        return $._erc1155TransferExempt[target_];
    }

    /**
     * @dev Approve `operator` to operate on all of `owner` tokens
     *
     * Emits an {ApprovalForAll} event.
     *
     * Requirements:
     *
     * - `operator` cannot be the zero address.
     */
    function _erc1155SetApprovalForAll(
        address owner,
        address operator,
        bool approved
    ) internal virtual {
        ERC404Storage storage $ = _getERC404Storage();
        if (operator == address(0)) {
            revert ERC1155InvalidOperator(address(0));
        }
        $._operatorApprovals[owner][operator] = approved;
        emit ApprovalForAll(owner, operator, approved);
    }

    /**
     * @dev Transfers a `value` amount of tokens of type `id` from `from` to `to`. Will mint (or burn) if `from`
     * (or `to`) is the zero address.
     *
     * Emits a {TransferSingle} event if the arrays contain one element, and {TransferBatch} otherwise.
     *
     * Requirements:
     *
     * - If `to` refers to a smart contract, it must implement either {IERC1155Receiver-onERC1155Received}
     *   or {IERC1155Receiver-onERC1155BatchReceived} and return the acceptance magic value.
     * - `ids` and `values` must have the same length.
     *
     * NOTE: The ERC-1155 acceptance check is not performed in this function. See {_updateWithAcceptanceCheck} instead.
     */
    function _erc1155Update(
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory values
    ) internal virtual {
        ERC404Storage storage $ = _getERC404Storage();
        if (ids.length != values.length) {
            revert ERC1155InvalidArrayLength(ids.length, values.length);
        }

        address operator = _msgSender();

        for (uint256 i = 0; i < ids.length; ++i) {
            uint256 id = ids.unsafeMemoryAccess(i);
            uint256 value = values.unsafeMemoryAccess(i);
            if (from != address(0)) {
                uint256 fromBalance = $._erc1155Balances[id][from];
                if (fromBalance < value) {
                    revert ERC1155InsufficientBalance(
                        from,
                        fromBalance,
                        value,
                        id
                    );
                }
                unchecked {
                    // Overflow not possible: value <= fromBalance
                    $._erc1155Balances[id][from] = fromBalance - value;
                }
            }

            if (to != address(0)) {
                $._erc1155Balances[id][to] += value;
            }
        }

        if (ids.length == 1) {
            uint256 id = ids.unsafeMemoryAccess(0);
            uint256 value = values.unsafeMemoryAccess(0);
            emit TransferSingle(operator, from, to, id, value);
        } else {
            emit TransferBatch(operator, from, to, ids, values);
        }
    }

    /**
     * @dev Version of {_update} that performs the token acceptance check by calling
     * {IERC1155Receiver-onERC1155Received} or {IERC1155Receiver-onERC1155BatchReceived} on the receiver address if it
     * contains code (eg. is a smart contract at the moment of execution).
     *
     * IMPORTANT: Overriding this function is discouraged because it poses a reentrancy risk from the receiver. So any
     * update to the contract state after this function would break the check-effect-interaction pattern. Consider
     * overriding {_update} instead.
     */
    function _erc1155UpdateWithAcceptanceCheck(
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory values,
        bytes memory data
    ) internal virtual {
        _erc1155Update(from, to, ids, values);
        if (to != address(0)) {
            address operator = _msgSender();
            if (ids.length == 1) {
                uint256 id = ids.unsafeMemoryAccess(0);
                uint256 value = values.unsafeMemoryAccess(0);
                _erc1155DoSafeTransferAcceptanceCheck(
                    operator,
                    from,
                    to,
                    id,
                    value,
                    data
                );
            } else {
                _erc1155DoSafeBatchTransferAcceptanceCheck(
                    operator,
                    from,
                    to,
                    ids,
                    values,
                    data
                );
            }
        }
    }

    /**
     * @dev Transfers a `value` tokens of token type `id` from `from` to `to`.
     *
     * Emits a {TransferSingle} event.
     *
     * Requirements:
     *
     * - `to` cannot be the zero address.
     * - `from` must have a balance of tokens of type `id` of at least `value` amount.
     * - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
     * acceptance magic value.
     */
    function _erc1155SafeTransferFrom(
        address from,
        address to,
        uint256 id,
        uint256 value,
        bytes memory data
    ) internal {
        if (to == address(0)) {
            revert ERC1155InvalidReceiver(address(0));
        }
        if (from == address(0)) {
            revert ERC1155InvalidSender(address(0));
        }

        (
            uint256[] memory ids,
            uint256[] memory values
        ) = _erc1155AsSingletonArrays(id, value);
        _erc1155UpdateWithAcceptanceCheck(from, to, ids, values, data);
    }

    /**
     * @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_safeTransferFrom}.
     *
     * Emits a {TransferBatch} event.
     *
     * Requirements:
     *
     * - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
     * acceptance magic value.
     * - `ids` and `values` must have the same length.
     */
    function _erc1155SafeBatchTransferFrom(
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory values,
        bytes memory data
    ) internal {
        if (to == address(0)) {
            revert ERC1155InvalidReceiver(address(0));
        }
        if (from == address(0)) {
            revert ERC1155InvalidSender(address(0));
        }
        _erc1155UpdateWithAcceptanceCheck(from, to, ids, values, data);
    }

    /**
     * @dev Sets a new URI for all token types, by relying on the token type ID
     * substitution mechanism
     * https://eips.ethereum.org/EIPS/eip-1155#metadata[defined in the EIP].
     *
     * By this mechanism, any occurrence of the `\{id\}` substring in either the
     * URI or any of the values in the JSON file at said URI will be replaced by
     * clients with the token type ID.
     *
     * For example, the `https://token-cdn-domain/\{id\}.json` URI would be
     * interpreted by clients as
     * `https://token-cdn-domain/000000000000000000000000000000000000000000000000000000000004cce0.json`
     * for token type ID 0x4cce0.
     *
     * See {uri}.
     *
     * Because these URIs cannot be meaningfully represented by the {URI} event,
     * this function emits no events.
     */
    function _erc1155SetURI(string memory newuri) internal virtual {
        ERC404Storage storage $ = _getERC404Storage();
        $._uri = newuri;
    }

    /**
     * @dev Creates a `value` amount of tokens of type `id`, and assigns them to `to`.
     *
     * Emits a {TransferSingle} event.
     *
     * Requirements:
     *
     * - `to` cannot be the zero address.
     * - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155Received} and return the
     * acceptance magic value.
     */
    function _erc1155Mint(
        address to,
        uint256 id,
        uint256 value,
        bytes memory data
    ) internal {
        if (to == address(0)) {
            revert ERC1155InvalidReceiver(address(0));
        }
        (
            uint256[] memory ids,
            uint256[] memory values
        ) = _erc1155AsSingletonArrays(id, value);
        _erc1155UpdateWithAcceptanceCheck(address(0), to, ids, values, data);
    }

    /**
     * @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_mint}.
     *
     * Emits a {TransferBatch} event.
     *
     * Requirements:
     *
     * - `ids` and `values` must have the same length.
     * - `to` cannot be the zero address.
     * - If `to` refers to a smart contract, it must implement {IERC1155Receiver-onERC1155BatchReceived} and return the
     * acceptance magic value.
     */
    function _erc1155MintBatch(
        address to,
        uint256[] memory ids,
        uint256[] memory values,
        bytes memory data
    ) internal {
        if (to == address(0)) {
            revert ERC1155InvalidReceiver(address(0));
        }
        _erc1155UpdateWithAcceptanceCheck(address(0), to, ids, values, data);
    }

    /**
     * @dev Destroys a `value` amount of tokens of type `id` from `from`
     *
     * Emits a {TransferSingle} event.
     *
     * Requirements:
     *
     * - `from` cannot be the zero address.
     * - `from` must have at least `value` amount of tokens of type `id`.
     */
    function _erc1155Burn(address from, uint256 id, uint256 value) internal {
        if (from == address(0)) {
            revert ERC1155InvalidSender(address(0));
        }
        (
            uint256[] memory ids,
            uint256[] memory values
        ) = _erc1155AsSingletonArrays(id, value);
        _erc1155UpdateWithAcceptanceCheck(from, address(0), ids, values, "");
    }

    /**
     * @dev xref:ROOT:erc1155.adoc#batch-operations[Batched] version of {_burn}.
     *
     * Emits a {TransferBatch} event.
     *
     * Requirements:
     *
     * - `from` cannot be the zero address.
     * - `from` must have at least `value` amount of tokens of type `id`.
     * - `ids` and `values` must have the same length.
     */
    function _erc1155BurnBatch(
        address from,
        uint256[] memory ids,
        uint256[] memory values
    ) internal {
        if (from == address(0)) {
            revert ERC1155InvalidSender(address(0));
        }
        _erc1155UpdateWithAcceptanceCheck(from, address(0), ids, values, "");
    }

    /**
     * @dev Performs an acceptance check by calling {IERC1155-onERC1155Received} on the `to` address
     * if it contains code at the moment of execution.
     */
    function _erc1155DoSafeTransferAcceptanceCheck(
        address operator,
        address from,
        address to,
        uint256 id,
        uint256 value,
        bytes memory data
    ) internal {
        if (to.code.length > 0) {
            try
                IERC1155Receiver(to).onERC1155Received(
                    operator,
                    from,
                    id,
                    value,
                    data
                )
            returns (bytes4 response) {
                if (response != IERC1155Receiver.onERC1155Received.selector) {
                    // Tokens rejected
                    revert ERC1155InvalidReceiver(to);
                }
            } catch (bytes memory reason) {
                if (reason.length == 0) {
                    // non-ERC1155Receiver implementer
                    revert ERC1155InvalidReceiver(to);
                } else {
                    /// @solidity memory-safe-assembly
                    assembly {
                        revert(add(32, reason), mload(reason))
                    }
                }
            }
        }
    }

    /**
     * @dev Performs a batch acceptance check by calling {IERC1155-onERC1155BatchReceived} on the `to` address
     * if it contains code at the moment of execution.
     */
    function _erc1155DoSafeBatchTransferAcceptanceCheck(
        address operator,
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory values,
        bytes memory data
    ) internal {
        if (to.code.length > 0) {
            try
                IERC1155Receiver(to).onERC1155BatchReceived(
                    operator,
                    from,
                    ids,
                    values,
                    data
                )
            returns (bytes4 response) {
                if (
                    response != IERC1155Receiver.onERC1155BatchReceived.selector
                ) {
                    // Tokens rejected
                    revert ERC1155InvalidReceiver(to);
                }
            } catch (bytes memory reason) {
                if (reason.length == 0) {
                    // non-ERC1155Receiver implementer
                    revert ERC1155InvalidReceiver(to);
                } else {
                    /// @solidity memory-safe-assembly
                    assembly {
                        revert(add(32, reason), mload(reason))
                    }
                }
            }
        }
    }

    /**
     * @dev Creates an array in memory with only one value for each of the elements provided.
     */
    function _erc1155AsSingletonArrays(
        uint256 element1,
        uint256 element2
    ) private pure returns (uint256[] memory array1, uint256[] memory array2) {
        /// @solidity memory-safe-assembly
        assembly {
            // Load the free memory pointer
            array1 := mload(0x40)
            // Set array length to 1
            mstore(array1, 1)
            // Store the single element at the next word after the length (where content starts)
            mstore(add(array1, 0x20), element1)

            // Repeat for next array locating it right after the first array
            array2 := add(array1, 0x40)
            mstore(array2, 1)
            mstore(add(array2, 0x20), element2)

            // Update the free memory pointer by pointing after the second array
            mstore(0x40, add(array2, 0x40))
        }
    }

    function _transfer(
        address from_,
        address to_,
        uint256 value_
    ) internal virtual returns (bool) {
        ERC404Storage storage $ = _getERC404Storage();
        uint256 senderBalance = balanceOf(from_);
        uint256 receiverBalance = balanceOf(to_);

        _erc20Transfer(from_, to_, value_);

        bool isFromExempt = erc1155TransferExempt(from_);
        bool isToExempt = erc1155TransferExempt(to_);

        if (isFromExempt && isToExempt) return true;

        bool isSendOnly = isFromExempt == false && isToExempt;
        bool isReceiveOnly = isFromExempt && isToExempt == false;
        bool isTransfer = isFromExempt == false && isToExempt == false;
        if (isSendOnly) {
            uint256 b = (senderBalance / $._units) -
                (balanceOf(from_) / $._units);
            _erc1155Burns(from_, $._ids, b);
        } else if (isReceiveOnly) {
            uint256 m = (balanceOf(to_) / $._units) -
                (receiverBalance / $._units);
            _erc1155Mints(to_, $._ids, m);
        } else if (isTransfer) {
            uint256 s = (senderBalance / $._units) -
                (balanceOf(from_) / $._units);
            uint256 r = (balanceOf(to_) / $._units) -
                (receiverBalance / $._units);
            _erc1155SafeTransfersFrom(from_, to_, $._ids, s);
            if (s < r) {
                //mint receiver
                uint256 m = r - s;
                _erc1155Mints(to_, $._ids, m);
            }
        }
        return true;
    }

    function erc1155BalanceOf(
        address account_,
        uint256 id_
    ) public view virtual returns (uint256) {
        ERC404Storage storage $ = _getERC404Storage();
        return $._erc1155Balances[id_][account_];
    }

    function _erc1155SafeTransfersFrom(
        address from_,
        address to_,
        uint256[] memory ids_,
        uint256 total_
    ) internal {
        ERC1155Data[] memory res = shuffle(from_, ids_, total_);
        for (uint256 i = 0; i < res.length; i++) {
            if (res[i].value > 0) {
                _erc1155SafeTransferFrom(
                    from_,
                    to_,
                    res[i].id,
                    res[i].value,
                    ""
                );
            }
        }
    }

    function _erc1155Burns(
        address from_,
        uint256[] memory ids_,
        uint256 total_
    ) internal {
        ERC1155Data[] memory res = shuffle(from_, ids_, total_);
        for (uint256 i = 0; i < res.length; i++) {
            if (res[i].value > 0) {
                _erc1155Burn(from_, res[i].id, res[i].value);
            }
        }
    }

    function _erc1155Mints(
        address to_,
        uint256[] memory ids_,
        uint256 total_
    ) internal {
        ERC1155Data[] memory res = shuffle(address(0), ids_, total_);
        for (uint256 i = 0; i < res.length; i++) {
            if (res[i].value > 0) {
                _erc1155Mint(to_, res[i].id, res[i].value, "");
            }
        }
    }

    function shuffle(
        address account_,
        uint256[] memory ids_,
        uint256 total_
    ) public view returns (ERC1155Data[] memory ret) {
        if (ids_.length == 1) {
            ret = new ERC1155Data[](1);
            ret[0] = ERC1155Data({id: ids_[0], value: total_});
            return ret;
        }

        uint256 t = 0;
        uint256 counter = 0;
        uint j = 0;
        bytes32 b32 = keccak256(abi.encodePacked(block.timestamp + counter));
        uint length = ids_.length;
        ret = new ERC1155Data[](length);
        // ret = [{id: 0, value: 0}, {id: 1, value: 0},...]
        for (uint256 i = 0; i < ids_.length; i++) {
            ret[i] = ERC1155Data({id: ids_[i], value: 0});
        }

        // max = [{id: 0, value: balance_of_account}, {id: 1, value: balance_of_account},...]
        ERC1155Data[] memory max = new ERC1155Data[](length);
        for (uint256 i = 0; i < ids_.length; i++) {
            max[i] = ERC1155Data({
                id: ids_[i],
                value: erc1155BalanceOf(account_, ids_[i])
            });
        }

        while (t < total_) {
            for (uint256 i = 0; i < ids_.length; i++) {
                if (t == total_) break;
                if (j > 31) {
                    b32 = keccak256(
                        abi.encodePacked(block.timestamp + ++counter)
                    );
                    j = 0;
                }
                uint8 value = uint8(b32[j++]);
                // n in [0,..,6]
                uint256 n = value % length;
                uint256 temp = ids_[n];
                ids_[n] = ids_[i];
                ids_[i] = temp;

                if (
                    account_ != address(0) &&
                    ret[ids_[i]].value == max[ids_[i]].value
                ) {
                    continue;
                }
                ret[ids_[i]].value += 1;

                t++;
            }
        }
        return ret;
    }

    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override returns (bool) {
        return
            interfaceId == type(IERC404).interfaceId ||
            interfaceId == type(IERC20).interfaceId ||
            interfaceId == type(IERC1155).interfaceId ||
            super.supportsInterface(interfaceId);
    }
}
