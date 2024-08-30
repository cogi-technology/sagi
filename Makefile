VERBOSE := $(if ${CI},--verbose,)
CARGO := cargo

test:
	${CARGO} test ${VERBOSE} --all -- --nocapture

# doc:
# 	cargo doc --all --no-deps \
# 	&& docker run --rm \
# 		-v ./docs:/out \
# 		-v ./lib/service/proto/src/proto/v1:/protos \
# 		pseudomuto/protoc-gen-doc --doc_opt=markdown,proto.md \
# 	&& docker run --rm \
# 		-v ./docs:/out \
# 		-v ./lib/service/proto/src/proto/v1:/protos \
# 		pseudomuto/protoc-gen-doc --doc_opt=html,proto.html

doc-deps:
	cargo doc --all

check:
	${CARGO} check ${VERBOSE} --all

build-sagi-image:
	docker build . -t sagi:latest -f ./docker/Dockerfile

run-sagi-dev:
	RUST_BACKTRACE=full RUST_LOG=info,hyper=info,openapi=trace${RUST_LOG} ${CARGO} run --bin openapi-server

up-sagi-docker:
	cd ./docker && docker compose up -d

down-sagi-docker:
	cd ./docker && docker compose down && sudo rm -rf lib

check-fmt:
	cargo +nightly fmt ${VERBOSE} --all -- --check

fmt:
	cargo +nightly fmt ${VERBOSE} --all

clippy:
	${CARGO} clippy ${VERBOSE} --all --all-targets --all-features -- \
		-D warnings -D clippy::enum_glob_use -D clippy::clone_on_ref_ptr

sort:
	cargo sort -gw

check-sort:
	cargo sort -gwc

ci: fmt check-fmt clippy test

info:
	date
	pwd
	env

# For counting lines of code
stats:
	@cargo count --version || cargo +nightly install --git https://github.com/kbknapp/cargo-count
	@cargo count --separator , --unsafe-statistics

# Use cargo-audit to audit Cargo.lock for crates with security vulnerabilities
# expecting to see "Success No vulnerable packages found"
security-audit:
	@cargo audit --version || cargo install cargo-audit
	@cargo audit

.PHONY: build
.PHONY: fmt test clippy doc doc-deps check stats
.PHONY: ci info security-audit