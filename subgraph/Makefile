generate-subgraph:
	node ./config/generate-networks.js && node ./config/generate-subgraph.js

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

run-graph-node:
	sudo rm -rf data && \
	docker compose up -d

down-graph-node:
	docker compose down && \
	sudo rm -rf data

subgraph-deploy:
	cd ./subgraphs && \
		npm run create-local && \
		npm run deploy-local