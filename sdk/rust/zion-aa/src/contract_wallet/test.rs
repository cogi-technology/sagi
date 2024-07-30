use ethers::{
    signers::{LocalWallet, Signer},
    types::Address,
};
use jsonwebtoken::TokenData;
use serde_json::json;

use crate::{
    constants::{get_contract_wallet_operator, Networkish},
    contract_wallet::{
        client::{Client, ClientMethods},
        operator::Operator,
        wallet::ContractWallet,
    },
    types::{jwt::JWTOptions, login::LoginData},
    utils::decode_jwt,
};
use std::sync::Arc;

#[tokio::test]
async fn test_start_contract_wallet() {
    let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImlHS1VBT05tQ0RBUU5oQXVCNHFhOUtCaiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MjIzMzQzNjcsImV4cCI6MTcyMjQyMDc2NywibmJmIjoxNzIyMzM0MzY3LCJpc3MiOiJodHRwczovL2lkLnRlbGVncmFtLm9yZyIsInN1YiI6IjcxMDk3NDA0ODIiLCJhdWQiOiI3MTA5NzQwNDgyIn0.XFDTx_hlLg4nT4rEaVLItqH_6TZ3PgnTmO7yiAtxN7rIEZEi0BlmkLw6M7dRsi7UXvtsvt49vrodlS0BkztpyK4Qs2BNWFckcUzjVoLLkiTRKG6j2QmKjqKidbJlf2N2vjEhhNh0__vd7BpEdyhqkl6qCLIsm-8MvOd1vEwZmfvMGAZyvGmeHLvVb911w50drlbUpn3yMiVAEoybfqCS20pYYfj3-oYg1tO0ZyUofgNZK0uBmMcvV7RYnNgKpGjW4JYtb-qLmY7Ly3EtM4lMiZFSWCv6JdxZETG9dQ35x7QTUxFPCAM_YjdAlNm8EgkSgHPSVptZeqLEE_M_usaT4Q";
    let data = json!({
        "salt": "0x54e4fa388e3436173d9c500b172b83d0d5a68019c20955c4f2c8f6554213faeb",
        "proof": {
            "pi_a": [
                "4492760939276083950493364964567969550774358476052568390543484310556636469905",
                "8504783926128909277272229396928202594384881761336931149776843740611407734906",
                "1"
            ],
            "pi_b": [
                [
                    "5506316460306732031529490405061437571753637458407835973393119729568883342930",
                    "3823457462917006896398908721180538799374115906915261657137040734677476977458"
                ],
                [
                    "21179192437064425489560654856660632630722939281324375343946458203182581449100",
                    "4880540644092945842974649772688802795435516072680605224756502405757835803159"
                ],
                [
                    "1",
                    "0"
                ]
            ],
            "pi_c": [
                "19211671521757716373734307813238970263078324430084990413786998208114083192676",
                "19147864884536841856841715460468240616536521700066947631879423752627145216361",
                "1"
            ],
            "protocol": "groth16"
        },
        "ephemeral_key_pair": "0xd9c0e80bb62c47ccbfc65e4c40f1470d8d00ae09919346d0454a6991bac4e239",
        "beneficiaries": [
            "0xfe39693d77c7c83e26ff7df39c13fc36f9cc88f5"
        ]
    });

    let toke_data = decode_jwt(token).unwrap();
    let login_data = serde_json::from_value::<LoginData>(data).unwrap();
    let beneficiaries = login_data
        .beneficiaries
        .iter()
        .map(|b| b.parse::<Address>().unwrap())
        .collect::<Vec<_>>();
    let jwt_options =
        JWTOptions::<LocalWallet>::try_into(toke_data.clone(), login_data.clone()).unwrap();

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
        .await
        .unwrap();

    let mut contract_wallet = ContractWallet::<Client, _>::new(contract_wallet_address, operator);
    contract_wallet.set_jwt(jwt_options);

    assert!(contract_wallet.is_writeable().await);
}

#[test]
fn decode_salt() {
    let salt = "54e4fa388e3436173d9c500b172b83d0d5a68019c20955c4f2c8f6554213faeb";
    let mut salt_in_hex = [0u8; 32];
    hex::decode_to_slice(salt, &mut salt_in_hex).unwrap();

    println!("{:?}", salt_in_hex);
    println!("{:?}", salt_in_hex.len());

    let wallet = "0xd9c0e80bb62c47ccbfc65e4c40f1470d8d00ae09919346d0454a6991bac4e239"
        .parse::<LocalWallet>()
        .unwrap();
    println!("{:#x}", wallet.address());
}
