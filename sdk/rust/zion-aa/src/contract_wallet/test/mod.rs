use {
    crate::{
        constants::{get_contract_wallet_operator, Networkish},
        contract_wallet::{client::Client, operator::Operator, wallet::ContractWallet},
        types::{jwt::JWTOptions, request::AuthorizationData},
        utils::decode_jwt,
    },
    anyhow::{anyhow, Result},
    ethers::{
        signers::LocalWallet,
        types::{Address, BlockNumber, Eip1559TransactionRequest},
    },
    ethers_providers::Middleware,
    jsonwebtoken::TokenData,
    std::{io::BufReader, sync::Arc},
};

#[tokio::test]
async fn test_contract_wallet_address_correct() -> Result<()> {
    let token = std::fs::read_to_string("./src/contract_wallet/test/inputs/jwt.data")?;
    let file = std::fs::File::open("./src/contract_wallet/test/inputs/login-data.json")?;
    let reader = BufReader::new(file);
    let AuthorizationData {
        salt,
        proof: _,
        ephemeral_key_pair,
        beneficiaries,
    } = serde_json::from_reader::<_, AuthorizationData>(reader)?;

    let toke_data = decode_jwt(&token)?;

    let beneficiaries = beneficiaries
        .iter()
        .map(|b| b.parse::<Address>().unwrap())
        .collect::<Vec<_>>();

    let contract_wallet_operator =
        get_contract_wallet_operator(Some(Networkish::Name("ziontestnet".into())));

    let rpc_endpoint = "https://torii.zionx.network/";
    let client = Arc::new(
        Operator::<Client>::get_ephemeral_key_pair(
            rpc_endpoint,
            contract_wallet_operator.chain_id,
            Some(ephemeral_key_pair.as_str()),
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

    let TokenData {
        header: _,
        claims: payload,
    } = toke_data.clone();
    let contract_wallet_address = operator
        .get_address(payload.sub, salt, payload.iss, payload.aud)
        .await?;
    let contract_wallet_address = format!("{:#x}", contract_wallet_address);

    assert_eq!(
        contract_wallet_address,
        "0x4819abcfe42c07e8957a4f42e1100c8473e27159"
    );

    Ok(())
}

#[tokio::test]
async fn test_create_wallet_is_ok() -> Result<()> {
    let token = std::fs::read_to_string("./src/contract_wallet/test/inputs/jwt.data")?;
    let file = std::fs::File::open("./src/contract_wallet/test/inputs/login-data.json")?;
    let reader = BufReader::new(file);
    let AuthorizationData {
        salt,
        proof,
        ephemeral_key_pair,
        beneficiaries,
    } = serde_json::from_reader::<_, AuthorizationData>(reader)?;

    let toke_data = decode_jwt(&token)?;

    let jwt_options = JWTOptions::<LocalWallet>::try_init(
        toke_data.clone(),
        ephemeral_key_pair.clone(),
        proof,
        salt.clone(),
    )?;

    let beneficiaries = beneficiaries
        .iter()
        .map(|b| b.parse::<Address>().unwrap())
        .collect::<Vec<_>>();

    let contract_wallet_operator =
        get_contract_wallet_operator(Some(Networkish::Name("ziontestnet".into())));

    let rpc_endpoint = "https://torii.zionx.network/";
    let client = Arc::new(
        Operator::<Client>::get_ephemeral_key_pair(
            rpc_endpoint,
            contract_wallet_operator.chain_id,
            Some(ephemeral_key_pair.as_str()),
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

    let TokenData {
        header: _,
        claims: payload,
    } = toke_data.clone();
    let contract_wallet_address = operator
        .get_address(payload.sub, salt, payload.iss, payload.aud)
        .await?;

    let mut contract_wallet = ContractWallet::<Client, _>::new(contract_wallet_address, operator);
    contract_wallet.set_jwt(jwt_options);

    assert!(contract_wallet.is_readonly().await);

    {
        let requested_prefund_create_wallet = contract_wallet.get_required_prefund()?;
        let wallet_balance = client
            .get_balance(contract_wallet_address, Some(BlockNumber::Latest.into()))
            .await?;
        if wallet_balance < requested_prefund_create_wallet {
            return Err(anyhow!("didn't pay prefund"));
        }
    }

    let status = contract_wallet
        .create(None, None)
        .await?
        .status
        .unwrap()
        .as_u64();
    assert_eq!(status, 1);

    Ok(())
}

#[tokio::test]
async fn test_validate_pin_code_is_ok() -> Result<()> {
    let token = std::fs::read_to_string("./src/contract_wallet/test/inputs/jwt.data")?;
    let file = std::fs::File::open("./src/contract_wallet/test/inputs/login-data.json")?;
    let reader = BufReader::new(file);
    let AuthorizationData {
        salt,
        proof,
        ephemeral_key_pair,
        beneficiaries,
    } = serde_json::from_reader::<_, AuthorizationData>(reader)?;

    let toke_data = decode_jwt(&token)?;

    let jwt_options = JWTOptions::<LocalWallet>::try_init(
        toke_data.clone(),
        ephemeral_key_pair.clone(),
        proof,
        salt.clone(),
    )?;

    let beneficiaries = beneficiaries
        .iter()
        .map(|b| b.parse::<Address>().unwrap())
        .collect::<Vec<_>>();

    let contract_wallet_operator =
        get_contract_wallet_operator(Some(Networkish::Name("ziontestnet".into())));

    let rpc_endpoint = "https://torii.zionx.network/";
    let client = Arc::new(
        Operator::<Client>::get_ephemeral_key_pair(
            rpc_endpoint,
            contract_wallet_operator.chain_id,
            Some(ephemeral_key_pair.as_str()),
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

    let TokenData {
        header: _,
        claims: payload,
    } = toke_data.clone();
    let contract_wallet_address = operator
        .get_address(payload.sub, salt, payload.iss, payload.aud)
        .await?;

    let mut contract_wallet = ContractWallet::<Client, _>::new(contract_wallet_address, operator);
    contract_wallet.set_jwt(jwt_options);

    assert!(contract_wallet.is_writeable().await);

    let code = "123456".to_string();
    let has_pin_code = contract_wallet.has_pin_code().await?;
    contract_wallet
        .validate_and_set_pin_code(code, !has_pin_code, None)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_transfer_native_token_via_contract_wallet() -> Result<()> {
    let token = std::fs::read_to_string("./src/contract_wallet/test/inputs/jwt.data")?;
    let file = std::fs::File::open("./src/contract_wallet/test/inputs/login-data.json")?;
    let reader = BufReader::new(file);
    let AuthorizationData {
        salt,
        proof,
        ephemeral_key_pair,
        beneficiaries,
    } = serde_json::from_reader::<_, AuthorizationData>(reader)?;

    let toke_data = decode_jwt(&token)?;

    let beneficiaries = beneficiaries
        .iter()
        .map(|b| b.parse::<Address>().unwrap())
        .collect::<Vec<_>>();

    let jwt_options = JWTOptions::<LocalWallet>::try_init(
        toke_data.clone(),
        ephemeral_key_pair.clone(),
        proof,
        salt,
    )?;

    let contract_wallet_operator =
        get_contract_wallet_operator(Some(Networkish::Name("ziontestnet".into())));

    let rpc_endpoint = "https://torii.zionx.network/";
    let client = Arc::new(
        Operator::<Client>::get_ephemeral_key_pair(
            rpc_endpoint,
            contract_wallet_operator.chain_id,
            Some(ephemeral_key_pair.as_str()),
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
        "0x4307E9f6cEd7aC3deC02dD90040F45034d55F8ab".parse::<Address>()?;
    println!("{:#x}", contract_wallet_address);

    let mut contract_wallet = ContractWallet::<Client, _>::new(contract_wallet_address, operator);
    contract_wallet.set_jwt(jwt_options);

    assert!(contract_wallet.is_writeable().await);

    let code = "123456".to_string();
    let has_pin_code = contract_wallet.has_pin_code().await?;
    contract_wallet
        .validate_and_set_pin_code(code, !has_pin_code, None)
        .await?;

    let dest = "0x053591Bc227838526c80aCF2400377F4822d8623".parse::<Address>()?;

    let before_bal_of_contract_wallet = client
        .get_balance(contract_wallet.address(), Some(BlockNumber::Latest.into()))
        .await?;
    let before_bal_of_wallet = client
        .get_balance(dest, Some(BlockNumber::Latest.into()))
        .await?;

    let transaction = Eip1559TransactionRequest::new()
        .to(dest)
        .value(ethers::utils::parse_ether("0.00001")?);

    let result = contract_wallet
        .send_transaction(transaction, None)
        .await
        .unwrap();

    let after_bal_of_contract_wallet = client
        .get_balance(contract_wallet.address(), Some(BlockNumber::Latest.into()))
        .await?;
    let after_bal_of_wallet = client
        .get_balance(dest, Some(BlockNumber::Latest.into()))
        .await?;

    println!(
        "balance of contract_wallet: before: {:#?} after: {:#?}",
        before_bal_of_contract_wallet, after_bal_of_contract_wallet
    );
    println!(
        "balance of wallet: before: {:#?} after: {:#?}",
        before_bal_of_wallet, after_bal_of_wallet
    );

    println!("{:#?}", result);

    Ok(())
}
