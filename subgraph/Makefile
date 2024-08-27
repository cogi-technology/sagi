IPFS := http://localhost:5001
NODE := http://localhost:8020/

configure-subgraph:
	node ./config/generate-networks.js && node ./config/generate-subgraph.js && node ./config/generate-docker-compose.js

# Build session 
erc20-subgraph-build:
	cd ./erc20 && sudo \
		sudo rm -rf build && sudo rm -rf generated && \
		graph codegen && graph build

erc721-subgraph-build:
	cd ./erc721 && sudo \
		sudo rm -rf build && sudo rm -rf generated && \
		graph codegen && graph build

erc404-subgraph-build:
	cd ./erc404 && sudo \
		sudo rm -rf build && sudo rm -rf generated && \
		graph codegen && graph build

subgraph-build: erc20-subgraph-build erc721-subgraph-build erc404-subgraph-build

# test session
erc20-subgraph-test:
	cd ./erc20 && \
		graph test -d

erc404-subgraph-test:
	cd ./erc404 && \
		graph test -d

erc721-subgraph-test:
	cd ./erc721 && \
		graph test -d

subgraph-test: erc20-subgraph-test erc721-subgraph-test erc404-subgraph-test

# deploy session 
erc20-subgraph-deploy:
	cd ./erc20 && \
		graph create --node ${NODE} erc20 && \
		graph deploy --node ${NODE} --ipfs ${IPFS} erc20

erc721-subgraph-deploy:
	cd ./erc721 && \
		graph create --node ${NODE} erc721 && \
		graph deploy --node ${NODE} --ipfs ${IPFS} erc721

erc404-subgraph-deploy:
	cd ./erc404 && \
		graph create --node ${NODE} erc404 && \
		graph deploy --node ${NODE} --ipfs ${IPFS} erc404

subgraph-deploy: erc20-subgraph-deploy erc721-subgraph-deploy erc404-subgraph-deploy

erc20-subgraph-remove:
	cd ./erc20 && \
		graph remove --node ${NODE} erc20

erc404-subgraph-remove:
	cd ./erc404 && \
		graph remove --node ${NODE} erc404

erc721-subgraph-remove:
	cd ./erc721 && \
		graph remove --node ${NODE} erc721

subgraph-remove: erc20-subgraph-remove erc404-subgraph-remove erc721-subgraph-remove

run-graph-node:
	sudo rm -rf data && \
	docker compose up -d

down-graph-node: subgraph-remove
	docker compose down && \
	sudo rm -rf data