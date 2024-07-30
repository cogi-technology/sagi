use crate::{
    constants::{get_contract_wallet_operator, Networkish},
    contract_wallet::{
        client::{Client, ClientMethods},
        operator::Operator,
        wallet::ContractWallet,
    },
};
use std::sync::Arc;

#[tokio::test]
async fn test_start_contract_wallet() {
    // Get JWT
    let contract_wallet_operator =
        get_contract_wallet_operator(Some(Networkish::Name("localhost".into())));

    let rpc_endpoint = "http://localhost:8545";
    let client = Arc::new(
        Client::random_wallet(rpc_endpoint, contract_wallet_operator.chain_id)
            .await
            .unwrap(),
    );
    print!("{:#x}", client.address());

    let beneficiaries = vec![];

    // Setup other expected calls as necessary
    let operator = Arc::new(Operator::new(
        contract_wallet_operator,
        Arc::clone(&client),
        beneficiaries,
    ));
    // let contract_wallet_address = operator.get_address(sub, salt, iss, aud).await.unwrap();

    // let wallet = ContractWallet::new(contract_wallet_address, operator);

    // assert!(wallet.is_writeable().await);
}
