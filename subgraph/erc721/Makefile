# generate-subgraph:
# 	node ./config/generate-subgraph.js

subgraph-build:
	sudo rm -rf build && sudo rm -rf generated && \
	yarn codegen && yarn build

subgraph-test: 
	npm test -d

run-graph-node:
	sudo rm -rf data && \
	docker compose up -d
	
subgraph-deploy:
	npm run create-local && \
	npm run deploy-local

down-graph-node:
	npm run remove-local && \
	docker compose down && \
	sudo rm -rf data
