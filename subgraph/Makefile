generate-subgraph:
	node ./config/generate-networks.js && node ./config/generate-subgraph.js

# Build session 
erc20-subgraph-build:
	cd ./erc20 && sudo \
		sudo rm -rf build && sudo rm -rf generated && \
		npm run codegen && npm run build

erc721-subgraph-build:
	cd ./erc721 && sudo \
		sudo rm -rf build && sudo rm -rf generated && \
		npm run codegen && npm run build

erc404-subgraph-build:
	cd ./erc404 && sudo \
		sudo rm -rf build && sudo rm -rf generated && \
		npm run codegen && npm run build

subgraph-build: erc20-subgraph-build erc721-subgraph-build erc404-subgraph-build

# deploy session 
erc20-subgraph-deploy:
	cd ./erc20 && \
		npm run create-local && \
		npm run deploy-local

erc721-subgraph-deploy:
	cd ./erc721 && \
		npm run create-local && \
		npm run deploy-local

erc404-subgraph-deploy:
	cd ./erc404 && \
		npm run create-local && \
		npm run deploy-local

subgraph-deploy: erc20-subgraph-deploy erc721-subgraph-deploy erc404-subgraph-deploy


run-graph-node:
	sudo rm -rf data && \
	docker compose up -d

down-graph-node:
	docker compose down && \
	sudo rm -rf data