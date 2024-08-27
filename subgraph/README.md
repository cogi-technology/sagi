# Subgraph Deployment Guide for Non-Web3 Developers
## Introduction
This guide will walk you through the steps required to update and deploy a subgraph for smart contracts. You don't need prior experience with blockchain or Web3 development to follow this guide.

## Prerequisites
Before getting started, ensure you have the following installed on your machine:

1. Docker: Download it from [Install Docker](https://docs.docker.com/engine/install/).
2. Node.js (version 20.x or 18.x or 16.x recommended)
3. npm (comes with Node.js)
4. Graph CLI (Command Line Interface tool from The Graph)

    To install Graph CLI globally, run:
    ```bash
    npm install -g @graphprotocol/graph-cli
    ```

## Step 1: Setup enviroment
1. Clone the subgraph repository to your local machine:
    ```bash
    git clone git@github.com:cogi-technology/sagi.git && cd ./sagi/subgraph
    ```
    This downloads the project files to your computer.

2. Install the needed project dependencies:
    ```bash
    cd config && npm install
    ```
    ```bash
    cd erc20 && npm install
    ```
    ```bash
    cd erc404 && npm install
    ```
    ```bash
    cd erc721 && npm install
    ```
    This installs the necessary packages

## Step 2: Configuration for Deployment
1. Config the `config/config.yaml` file in a text editor.
2. Update the listed fields of the ERC-20 | ERC-721 | ERC-404 contract you want to track.
    ```yaml
    erc20:
        - name: <name contract>
          address: <address contract>
          startBlock: <start block of contract>
    ...
    ```
    Besides, you can also be unable to index the contract by commenting the whole block:
    ```yaml
    ## erc20:
    ##    - name: <name contract>
    ##      address: <address contract>
    ##      startBlock: <start block of contract>
    ...
    ```
    2.2. If you want to config ipfs, you can config ipfs in `config/config.yaml` file, else you can ignore this step.
    ```yaml
    ipfs:
        host: <ipfs host>
        port: <ipfs port>
    ```

3. Run makefile to update the `subgraph.yaml` file in subgraphs.
    ```bash
    make configure-subgraph
    ```

## Step 3: Codegen and Build the Subgraph
After configuring your subgraphs, you'll need to build them. This involves generating code and building the subgraph for deployment.
1. In your terminal, run:
    ```bash
    make subgraph-build
    ```
    - `graph codegen`: Generates TypeScript code based on your subgraph schema and smart contract ABIs.
    - `graph build`:  Compiles the subgraph and prepares it for deployment.

    Individual Subgraph Build Commands:
    - Build only ERC20 subgraph:
        ```bash
        make subgraph-build-erc20
        ```
    - Build only ERC721 subgraph:
        ```bash
        make subgraph-build-erc721
        ```
    - Build only ERC404 subgraph:
        ```bash
        make subgraph-build-erc404
        ```

2. Ensure that there are no errors in the terminal output. If there are errors, double-check the contract addresses and any changes you made in subgraph.yaml.

## Step 4: Deploy the Subgraph
After building, you can deploy your subgraphs to the Graph Node. This step requires both the IPFS and Graph Node to be running.
### Set Up Graph Node
1. Start the Graph Node:
    ```bash
    make run-graph-node
    ```
    Wait for the Graph Node to start. You should see logs like this:
    ```
    ...
    INFO Starting JSON-RPC admin server at: http://localhost:8020, component: JsonRpcServer
    ...
    ```

### Deploy to Graph Nodde
1. Run the following command to deploy the subgraph:
    ```bash
    make subgraph-deploy
    ```

### Stop the Graph Node
1. To stop the Graph Node, run:
    ```bash
    make down-graph-node
    ```

## Conclusion
Congratulations! You've successfully updated and deployed a subgraph by changing the contract addresses. If you encounter any issues, double-check the steps above or reach out to a Web3 developer for assistance.

**[Example ERC20 Queries](./erc20/queries.md)**

**[Example ERC721 Queries](./erc721/queries.md)**

**[Example ERC404 Queries](./erc404/queries.md)**