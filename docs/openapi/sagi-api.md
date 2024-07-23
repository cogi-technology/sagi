# SAGI OPENAPIs

## ERC20

- `deploy`: **[POST]** /api/erc20/deploy
  - body:
    ```json
    {
        "owner": "string",
        "name": "string",
        "symbol": "string",
        "decimals": "number",
        "initialSupply": "string"
    }
    ```
  - res:
    ```json
    {
      "contract": "string"
    }
    ```
  
- `totalSuppy`: **[GET]** /api/erc20/totalSupply?contract={ContractAddress}
  - res:
    ```json
    {
      "totalSupply": "string"
    }
    ```

- `balanceOf`: **[GET]** api/erc20/balanceOf?contract={ContractAddress}&account={AccountAddress}
  - res:
    ```json
    {
      "amount": "string"
    }
    ```

- `allowance`: **[GET]** api/erc20/allowance?contract={ContractAddress}&owner={OwnerAddress}&spender={SpenderAddress}
  - res:
    ```json
    {
      "remainingAmount": "string"
    }
    ```

- `approve`: **[POST]** api/erc20/approve
  - body:
    ```json
    {
      "contract": "string",
      "spender": "string",
      "amount": "string"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `transfer`: **[POST]** api/erc20/transfer 
  - body:
    ```json
    {
      "contract": "string",
      "recipient": "string",
      "amount": "string"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `transferFrom`: **[POST]** api/erc20/transfer 
  - body:
    ```json
    {
      "contract": "string",
      "sender": "string",
      "recipient": "string",
      "amount": "string"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ``` 

## ERC721

- `deploy`: **[POST]** /api/erc721/deploy
  - body:
    ```json
    {
      "name": "string",
      "symbol": "string",
      "owner": "string"
    }
    ```
  - resposnse: 
    ```json
    {
      "contract": "string"
    }
    ```

- `balanceOf`: **[GET]** /api/erc721/balanceOf?contract={ContractAddress}&owner={OwnerAddress}
  - res:
    ```json
    {
      "amount": "string"
    }
    ```

- `ownerOf`: **[GET]** /api/erc721/ownerOf?contract={ContractAddress}&tokenId={TokenIdNumber}
  - res: 
    ```json 
    {
      "onwer": "string"
    }
    ```

- `safeTransferFrom`: **[POST]** /api/erc721/safeTransferFrom
  - body:
    ```json
    {
      "contract": "string",
      "from": "string",
      "to": "string",
      "tokenId": "string"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `transferFrom`: **[POST]** /api/erc721/transferFrom
  - body:
    ```json
    {
      "contract": "string",
      "from": "string",
      "to": "string",
      "tokenId": "string"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `approve`: **[POST]** /api/erc721/approve
  - body:
    ```json
    {
      "contract": "string",
      "to": "string",
      "tokenId": "string"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `getApproved`: **[GET]** /api/erc721/getApproved?contract={ContractAddress}&tokenId={TokenIdNumber}
  - res:
    ```json
    {
      "operator": "string"
    }
    ```

- `setApprovalForAll`: **[POST]** /api/erc721/setApprovalForAll
  - body:
    ```json
    {
      "contract": "string",
      "operator": "string",
      "approved": "boolean"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `isApprovedForAll`: **[GET]** /api/erc721/isApprovedForAll?contract={}&owner={}&operator={}
  - res:
    ```json
    {
      "result": "boolean"
    }
    ```

- `safeTransferFrom`: **[POST]** /api/erc721/safeTransferFrom
  - body:
    ```json
    {
      "from": "string",
      "to": "string",
      "tokenId": "string",
      "data": "bytes"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

## ERC404

- `deploy`: **[POST]** /api/erc404/deploy
  - body:
    ```json
    {
      "owner": "string",
      "name": "string",
      "symbol": "string",
      "initialSupply": "string",
      "units": "string",
      "ids": ["string"],
      "uri": "string"
    }
    ```

- `totalSupply`: **[GET]** /api/erc404/totalSupply?contract={}
  - res:
    ```json
    {
      "totalSupply": "string"
    }
    ```

- `balanceOf`: **[GET]** /api/erc404/totalSupply?contract={}&account={}
  - res:
    ```json
    {
      "amount": "string"
    }
    ```

- `allowance`: **[GET]** /api/erc404/allowance?contract={}&owner={}&spender={}
  - res:
    ```json
    {
      "remainingAmount": "string"
    }
    ```

- `approve`: **[POST]** /api/erc404/approve
  - body:
    ```json
    {
      "contract": "string",
      "spender": "string",
      "amount": "string"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `transfer`: **[POST]** /api/erc404/transfer 
  - body:
    ```json
    {
      "contract": "string",
      "recipient": "string",
      "amount": "string"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `transferFrom`: **[POST]** /api/erc404/transferFrom 
  - body:
    ```json
    {
      "contract": "string",
      "from": "string",
      "to": "string",
      "value": "string"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `balanceOfBatch`: **[GET]** /api/erc404/balanceOfBatch
  - param:
    ```json
    {
      "contract": "string",
      "owner": "string",
      "tokenIds": ["string"]
    }
    ```
  - res:
    ```json
    {
      "amounts": ["string"]
    }
    ```

- `setApprovalForAll`: **[POST]** /api/erc404/setApprovalForAll
  - body:
    ```json
    {
      "contract": "string",
      "operator": "string",
      "approved": "boolean"
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `isApprovedForAll`: **[GET]** /api/erc404/isApprovedForAll?contract={}&owner={}&operator={}
  - res:
    ```json
    {
      "result": "boolean"
    }
    ```

- `safeTransferFrom`: **[POST]** /api/erc404/safeTransferFrom
  - body:
    ```json
    {
      "from": "string",
      "to": "string",
      "tokenId": "string",
      "value": "string",
      "data": "bytes",
    }
    ```
  - res:
    ```json
    {
      "txhash"
    }
    ```

- `safeBatchTransferFrom`: **[POST]** /api/erc404/safeBatchTransferFrom
  - body:
    ```json
    {
      "from": "string",
      "to": "string",
      "tokenIds": ["string"],
      "values": ["string"],
      "data": "bytes",
    }
    ```
  - res:
    ```json
    {
      "txhash": "string"
    }
    ```

- `erc1155BalanceOf`: **[GET]** /api/erc404/erc1155BalanceOf?contract={}&account={}&tokenId={}
  - res:
    ```json
    {
      "amount": "string"
    }
    ```

- `erc20BalanceOf`: **[GET]** /api/erc404/erc20BalanceOf?contract={}&account={}
  - res:
    ```json
    {
      "amount": "string"
    }
    ```

- `erc1155TransferExempt`: **[GET]** /api/erc404/erc1155TransferExempt?target={}
  - res:
    ```json
    {
      "result": "boolean"
    }
    ```
