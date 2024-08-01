use {
    crate::{
        constants::{get_contract_wallet_operator, Networkish},
        contract_wallet::{
            client::{Client, ClientMethods},
            operator::Operator,
            wallet::ContractWallet,
        },
        types::{jwt::JWTOptions, login::LoginData},
        utils::decode_jwt,
    },
    anyhow::{anyhow, Result},
    ethers::{
        signers::LocalWallet,
        types::{Address, Eip1559TransactionRequest},
    },
    ethers_providers::Middleware,
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
    let client =
        Arc::new(Client::random_wallet(rpc_endpoint, contract_wallet_operator.chain_id).await?);

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
        .get_address(payload.sub, login_data.salt, payload.iss, payload.aud)
        .await?;

    println!("{:#x}", contract_wallet_address);

    let mut contract_wallet = ContractWallet::<Client, _>::new(contract_wallet_address, operator);
    contract_wallet.set_jwt(jwt_options);

    if contract_wallet.is_readonly().await {
        let request = contract_wallet.get_required_prefund()?;
        let wallet_balance = client
            .provider()
            .get_balance(contract_wallet.address(), None)
            .await?;

        if wallet_balance < request {
            println!("request: {}, wallet bal: {}", request, wallet_balance);
            return Err(anyhow!("didn't pay prefund"));
        }

        let _ = contract_wallet.create(None, None).await?;
    }

    assert!(contract_wallet.is_writeable().await);

    let transaction = Eip1559TransactionRequest::new()
        .to("0x053591Bc227838526c80aCF2400377F4822d8623".parse::<Address>()?)
        .value(ethers::utils::parse_ether("0.000001")?);

    let result = contract_wallet
        .send_transaction(transaction, None)
        .await
        .unwrap();

    println!("{:?}", result);

    Ok(())
}
