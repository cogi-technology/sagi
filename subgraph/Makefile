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
		docker compose down && \
		sudo rm -rf data

subgraph-deploy:
	cd ./subgraphs && \
		graph create --node http://localhost:8020/ subgraphs && \
		graph deploy --node http://localhost:8020/ --ipfs http://localhost:5001 subgraphs