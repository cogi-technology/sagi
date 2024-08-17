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

2. Install the project dependencies:
    ```bash
    cd config && npm install
    ```
    ```bash
    cd subgraphs && npm install
    ```
    This installs the necessary packages

## Step 2: Configuration in subgraph.yaml
1. Config the `config/config.yaml` file in a text editor.
2. Update the listed fields of the ERC-20 | ERC-721 | ERC-404 contract you want to track.
    ```yaml
    erc20:
        address: <address contract>
        startBlock: <start block of contract>
    ...
    ```
    Besides, you can also be unable to index the contract by commenting the whole block:
    ```yaml
    ## erc20:
    ##    address: <address contract>
    ##    startBlock: <start block of contract>
    ...
    ```

3. Run makefile to update the `subgraph.yaml` file in subgraphs.
    ```bash
    make generate-subgraph
    ```

## Step 3: Codegen and Build the Subgraph
After updating the needed fields for contracts, the next step is to build the subgraph.
1. In your terminal, run:
    ```bash
    make subgraph-build
    ```
    - `graph codegen`: Generates TypeScript code based on your subgraph schema and smart contract ABIs.
    - `graph build`:  Compiles the subgraph and prepares it for deployment.

2. Ensure that there are no errors in the terminal output. If there are errors, double-check the contract addresses and any changes you made in subgraph.yaml.

## Step 4: Deploy the Subgraph
Once the subgraph is built, you deploy it to Graph Node or the decentralized Graph Network.
### Set Up Graph Node
1. Start the Graph Node:
    ```bash
    make run-graph-node
    ```
    Wait for the Graph Node to start. You should see logs like this:
    ```
    ...
    2024-08-15 09:38:16 Aug 15 02:38:16.937 INFO Starting JSON-RPC admin server at: http://localhost:8020, component: JsonRpcServer
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

**[Example Queries](./example.md)**