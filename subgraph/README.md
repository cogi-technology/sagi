# 🚀 Subgraph Deployment Guide for Non-Web3 Developers
## 📚 Introduction
This guide will walk you through the steps required to update and deploy a subgraph for smart contracts. You don't need prior experience with blockchain or Web3 development to follow this guide.

## 🛠 Prerequisites
Before getting started, ensure you have the following installed on your machine:

1. Docker 🐳: Download it from [Docker's official website](https://www.docker.com/get-started/).
2. Node.js (version 20.x or 18.x or 16.x recommended) 🟢
3. npm (comes with Node.js) 📦
4. Graph CLI (Command Line Interface tool from The Graph)

    To install Graph CLI globally, run:
    ```bash
    npm install -g @graphprotocol/graph-cli
    ```

## 📝 Step 1: Clone the Subgraph Repository
1. Clone the subgraph repository to your local machine:
    ```bash
    git clone <repository_url>
    ```
    🖥️ This downloads the project files to your computer.


## 🔄 Step 2: Configuration in subgraph.yaml
1. Config the `config/config.yaml` file in a text editor. ✏️
2. Update the listed fields of the ERC-20 | ERC-721 | ERC-404 contract you want to track. 🔗
    ```yaml
    erc20:
        network: <rpc_url>
        address: <address contract>
        startBlock: <start block of contract>
    ...
    ```
    Besides, you can also be unable to index the contract by commenting the whole block:
    ```yaml
    ## erc20:
    ##    network: <rpc_url>
    ##    address: <address contract>
    ##    startBlock: <start block of contract>
    ...
    ```

3. Run makefile to update the `subgraph.yaml` file in subgraphs. 🔄
    ```bash
    make generate-subgraph
    ```

## 🛠️ Step 3: Codegen and Build the Subgraph
After updating the needed fields for contracts, the next step is to build the subgraph.
1. In your terminal, run:
    ```bash
    make subgraph-build
    ```
    - `graph codegen`: Generates TypeScript code based on your subgraph schema and smart contract ABIs. ⚙️
    - `graph build`:  Compiles the subgraph and prepares it for deployment. 🏗️

2. Ensure that there are no errors in the terminal output. If there are errors, double-check the contract addresses and any changes you made in subgraph.yaml. ✅

## 🚀 Step 4: Deploy the Subgraph
Once the subgraph is built, you deploy it to Graph Node or the decentralized Graph Network.
### 🐳 Set Up Graph Node
1. Start the Graph Node 🚀:
    ```bash
    make run-graph-node
    ```

### 🌐 Deploy to Graph Nodde
1. Run the following command to deploy the subgraph:
    ```bash
    make subgraph-deploy
    ```

## 🎉 Conclusion
Congratulations! 🎊 You've successfully updated and deployed a subgraph by changing the contract addresses. If you encounter any issues, double-check the steps above or reach out to a Web3 developer for assistance.

**[Example Queries](./example.md)**