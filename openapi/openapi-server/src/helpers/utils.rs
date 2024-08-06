use anyhow::Result as AnyhowResult;
use ethers::{signers::LocalWallet, types::Address};
use openapi_proto::zionauthorization_service::{GetDataRequestForZionResponse, ProofPoints};
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use std::result::Result::Ok;
use std::sync::Arc;
use std::{collections::HashMap, ops::Add};
use tonic::{Response, Status};
use zion_aa::{
    constants::{get_contract_wallet_operator, Networkish},
    contract_wallet::{
        client::{Client as ZionClient, ClientMethods},
        operator::Operator,
        wallet::ContractWallet,
    },
    types::{jwt::JWTOptions, login::LoginData},
};

use crate::services::zionauthorization::get_data_request_for_zion_logic;

use super::into::{proto_proofpoint_from, sdk_proofpoint_from};

pub type Result<T> = std::result::Result<T, tonic::Status>;

pub fn into_anyhow(err: anyhow::Error) -> Status {
    Status::new(tonic::Code::Aborted, format!("{}", err))
}

pub async fn send_request_json<T: Serialize + DeserializeOwned, U: Serialize>(
    client: &Client,
    method: Method,
    url: &str,
    body: Option<&U>,
    headers: Option<HashMap<String, String>>,
) -> Result<Response<T>> {
    let mut request: RequestBuilder = client.request(method, url);

    // Set the body if provided
    if let Some(b) = body {
        request = request.json(b);
    }

    // Set headers if provided
    if let Some(h) = headers {
        for (key, value) in h {
            request = request.header(&key, &value);
        }
    }

    let response = match request.send().await {
        Ok(response) => response,
        Err(e) => return Err(into_anyhow(e.into())),
    };
    return match response.json::<T>().await {
        Ok(response) => Ok(Response::new(response)),
        Err(e) => return Err(into_anyhow(e.into())),
    };
}

pub async fn send_request_text<U>(
    client: &Client,
    method: Method,
    url: &str,
    body: Option<&U>,
    headers: Option<HashMap<String, String>>,
) -> Result<Response<String>>
where
    U: Serialize,
{
    let mut request: RequestBuilder = client.request(method, url);

    // Set the body if provided
    if let Some(b) = body {
        request = request.json(b);
    }

    // Set headers if provided
    if let Some(h) = headers {
        for (key, value) in h {
            request = request.header(&key, &value);
        }
    }

    let response = match request.send().await {
        Ok(response) => response,
        Err(e) => return Err(into_anyhow(e.into())),
    };
    return match response.text().await {
        Ok(response) => Ok(Response::new(response)),
        Err(e) => return Err(into_anyhow(e.into())),
    };
}

pub async fn init_contract_wallet(
    header_metadata: &tonic::metadata::MetadataMap,
    rpc_endpoint: &str,
) -> AnyhowResult<ContractWallet<ZionClient, LocalWallet>> {
    // let (
    //     GetDataRequestForZionResponse {
    //         salt,
    //         proof,
    //         ephemeral_key_pair,
    //         beneficiaries,
    //     },
    //     token_data,
    // ) = get_data_request_for_zion_logic(header_metadata).await?;
    let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImpOVHRoNU80WHFyT0NuR1BkeGtEME5pOCIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MjI5MzM4MzMsImV4cCI6MTcyMzAyMDIzMywibmJmIjoxNzIyOTMzODMzLCJpc3MiOiJodHRwczovL2lkLnRlbGVncmFtLm9yZyIsInN1YiI6IjU3NDA4NDczOTkiLCJhdWQiOiI3MTA5NzQwNDgyIn0.Sog1eSp9MKaPxau9RV-mK9o89ZiCEe8T4qN6A4M6u87MRgRlOgJRau_LYzZ2Pnotcviwfo8L6CRj-7hEI-DDGm3h3cyQ4bT48PBciTzWGrlmGmwp4gRRZgfJyWpqYDUJlue_99ermhS3KMUPM2v5mM99Ya44pPFBEV-FVXXc2CSj_994mYfWT1lhKiVKeTtEzxDt7VDv0M25JksLcEA9-SVfyeM0ehQytGFjGrrawQjVCSqgFN8RwT5RyvFdcn7OwWPSoechbrgjQTNYbb3eiDLQjInrmtRZeslmFO5rMd3VCr_t3dQTF_IInwIHBdbBtHiklozq8aJDTwDnL3w75w";
    let token_data = zion_aa::utils::decode_jwt(token)?;

    let data = json!({
        "salt": "0x8b007c3425216674ebb4db21f7531a274fdf9e567173ef8d93d95a01375d26b0",
        "proof": {
            "protocol": "groth16",
            "pi_a": [
                "14801816815982897191214849187901646186557653634630976994621098935299565837678",
                "3107512675410124980504736542732965398118402483813760832504749775336079809255",
                "1"
            ],
            "pi_b": [
                [
                    "15206008218501371535230496796318291276435425082329694810376753231329964445333",
                    "12979529397566891189029734629963374195985491101285244255488826680679659512201"
                ],
                [
                    "10785241355847769928761783282267618023354984164670580353019210034647839861320",
                    "15204461572087244143185958420961060931649571485574934594464139847767949860674"
                ],
                [
                    "1",
                    "0"
                ]
            ],
            "pi_c": [
                "17810260805960448247673615359179766731050272460642672129728519214520556494701",
                "16900961044965525423879885912038078627949256248409680923890979761253678529942",
                "1"
            ]
        },
        "ephemeral_key_pair": "e8e7c8b1aaf0398854ee51eb72eed6dd6c90e21128ee60010fda6ea18f691a87",
        "beneficiaries": [
            "0xfe39693d77c7c83e26ff7df39c13fc36f9cc88f5"
        ]
    });
    let login_data = serde_json::from_value::<LoginData>(data).unwrap();

    let address_beneficiaries = login_data
        .beneficiaries
        .iter()
        .map(|b| b.parse::<Address>().unwrap())
        .collect::<Vec<_>>();
    // let proof = sdk_proofpoint_from(proof.unwrap());

    let contract_wallet_operator =
        get_contract_wallet_operator(Some(Networkish::Name("ziontestnet".into())));

    let client = Arc::new(
        ZionClient::try_new(
            rpc_endpoint,
            contract_wallet_operator.chain_id,
            &login_data.ephemeral_key_pair,
        )
        .await?,
    );
    // let login_data = LoginData {
    //     salt,
    //     proof,
    //     ephemeral_key_pair,
    //     beneficiaries,
    // };
    let jwt_options = JWTOptions::<LocalWallet>::try_init(token_data, login_data)?;

    let operator = Arc::new(Operator::new(
        contract_wallet_operator,
        Arc::clone(&client),
        address_beneficiaries,
    ));
    let contract_address = "0x31158C661D5a1266c7A7324EE9beBc84293a67B1".parse::<Address>()?;

    let mut contract_wallet = ContractWallet::<ZionClient, _>::new(contract_address, operator);
    contract_wallet.set_jwt(jwt_options);

    Ok(contract_wallet)
}
