use {
    crate::{
        constants::{get_contract_wallet_operator, Networkish},
        contract_wallet::{
            client::{Client, ClientMethods},
            operator::Operator,
            wallet::ContractWallet,
        },
        types::{
            jwt::JWTOptions,
            login::{self, LoginData},
        },
        utils::decode_jwt,
    },
    anyhow::{anyhow, Result},
    ethers::{
        signers::LocalWallet,
        types::{Address, Eip1559TransactionRequest},
    },
    jsonwebtoken::TokenData,
    std::{io::BufReader, sync::Arc},
};

#[tokio::test]
async fn test_init_contract_wallet() -> Result<()> {
    let token = std::fs::read_to_string("./src/contract_wallet/test/inputs/jwt.data")?;
    let file = std::fs::File::open("./src/contract_wallet/test/inputs/login-data.json")?;
    let reader = BufReader::new(file);
    let login_data = serde_json::from_reader::<_, LoginData>(reader)?;

    let toke_data = decode_jwt(&token)?;

    println!("token_data: {:#?}", toke_data);
    println!("login_data: {:#?}", login_data);

    let beneficiaries = login_data
        .beneficiaries
        .iter()
        .map(|b| b.parse::<Address>().unwrap())
        .collect::<Vec<_>>();
    let jwt_options = JWTOptions::<LocalWallet>::try_init(toke_data.clone(), login_data.clone())?;

    // Get JWT
    let contract_wallet_operator =
        get_contract_wallet_operator(Some(Networkish::Name("ziontestnet".into())));

    let rpc_endpoint = "https://torii.zionx.network/";
    let client = Arc::new(
        Operator::<Client>::get_ephemeral_key_pair(
            rpc_endpoint,
            contract_wallet_operator.chain_id,
            Some(login_data.ephemeral_key_pair.as_str()),
        )
        .await?,
    );
    println!("Client: {:#x}", client.address());

    // Setup other expected calls as necessary
    let operator = Arc::new(Operator::new(
        contract_wallet_operator,
        Arc::clone(&client),
        beneficiaries,
    ));

    // let TokenData {
    //     header: _,
    //     claims: payload,
    // } = toke_data.clone();
    // let contract_wallet_address = operator
    //     .get_address(payload.sub, login_data.salt, payload.iss, payload.aud)
    //     .await?;

    let contract_wallet_address =
        "0x31158C661D5a1266c7A7324EE9beBc84293a67B1".parse::<Address>()?;
    println!("{:#x}", contract_wallet_address);

    let mut contract_wallet = ContractWallet::<Client, _>::new(contract_wallet_address, operator);
    contract_wallet.set_jwt(jwt_options);
    assert!(contract_wallet.is_writeable().await);

    let code = "123456".to_string();
    let has_pin_code = contract_wallet.has_pin_code().await?;
    contract_wallet
        .validate_and_set_pin_code(code, !has_pin_code, None)
        .await?;

    let transaction = Eip1559TransactionRequest::new()
        .to("0x053591Bc227838526c80aCF2400377F4822d8623".parse::<Address>()?)
        .value(ethers::utils::parse_ether("0.00000000000001")?);

    let result = contract_wallet
        .send_transaction(transaction, None)
        .await
        .unwrap();

    println!("{:#?}", result);

    Ok(())
}
