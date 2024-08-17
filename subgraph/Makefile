generate-subgraph:
	node ./config/generate-subgraph.js

subgraph-build:
	cd ./subgraphs && \
		sudo rm -rf build && sudo rm -rf generated && \
		graph codegen && graph build

subgraph-test:
	cd ./subgraphs && \
		graph test -d

run-graph-node:
	cd ./subgraphs && \
		sudo rm -rf data && \
		docker compose up -d

down-graph-node:
	cd ./subgraphs && \
		npm run remove-local && \
		docker compose down && \
		sudo rm -rf data

subgraph-deploy:
	cd ./subgraphs && \
		npm run create-local && \
		npm run deploy-local