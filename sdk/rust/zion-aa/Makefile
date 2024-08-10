VERBOSE := $(if ${CI},--verbose,)
CARGO := cargo

test:
	${CARGO} test ${VERBOSE} --all -- --nocapture

check-fmt:
	cargo +nightly fmt ${VERBOSE} --all -- --check

fmt:
	cargo +nightly fmt ${VERBOSE} --all

clippy:
	${CARGO} clippy ${VERBOSE} --all --all-targets --all-features -- \
		-D warnings -D clippy::enum_glob_use -D clippy::clone_on_ref_ptr

all-test: fmt check-fmt clippy test

test-transfer-native-token:
	cargo test test_transfer_native_token_via_contract_wallet -- --nocapture

test-get-address:
	cargo test test_contract_wallet_address_correct -- --nocapture

test_create_wallet:
	cargo test test_create_wallet_is_ok -- --nocapture

test-validate-wallet:
	cargo test test_validate_pin_code_is_ok -- --nocapture

test-transfer-erc20-token:
	cargo test test_transfer_erc20_token_via_contract_wallet -- --nocapture

test-mint-erc721-token:
	cargo test test_mint_erc721_token_via_contract_wallet -- --nocapture