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
    types::{jwt::JWTOptions, request::AuthorizationData},
};

use crate::services::zionauthorization::get_data_request_for_zion_logic;

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
    let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImpOVHRoNU80WHFyT0NuR1BkeGtEME5pOCIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MjI5NjE4OTQsImV4cCI6MTcyMzA0ODI5NCwibmJmIjoxNzIyOTYxODk1LCJpc3MiOiJodHRwczovL2lkLnRlbGVncmFtLm9yZyIsInN1YiI6IjU3NDA4NDczOTkiLCJhdWQiOiI3MTA5NzQwNDgyIn0.igL0Z8LbR3awnox4slTvbBsqnnWJyUQ4p1qvcJYwnkdO5py4W3mTuaFzx0xw8ByECIuK0juU9uQIg7xZQZV_nUXk5M_S8snMvSo2nn3Q_TTqbgDHGqtATLPMEkKAqnVHQQooJ_CkqO97AHgoQCTWkgZGoOhDg773I-CO1RQ3xUI7K6PP96X2CNA02Ez4JrRNklaJ44eR46I7sgmQWMzQzFx7iy6niSNXue1rZIWZsjws8NUBYZuTn_b24V56RCZw__j8-9NAmqFv2vitIwDiNMHk96hJDllSsJXOmoYukuIR0467qlTRJrJqW5_4NiuB9DK4drpWuB1rUnLK00qoAg";
    let token_data = zion_aa::utils::decode_jwt(token)?;

    let data = json!({
        "salt": "0x8b007c3425216674ebb4db21f7531a274fdf9e567173ef8d93d95a01375d26b0",
        "proof": {
            "protocol": "groth16",
            "pi_a": [
                "848517426970185902836957632039262920819670960348695817755692514778205507177",
                "11234122044483938094114577390707656040020989890470535128512942449808019728442",
                "1"
            ],
            "pi_b": [
                {
                    "values": [
                        "2781913689848125025037491843906790085846254178558810253837255454141446402807",
                        "3647356397285913093210792537100269613673081645711183459275916670275549687181"
                    ]
                },
                {
                    "values": [
                        "6057743492438896040617107135360846068647417847934794073591042834408986579993",
                        "5161670272154989331230524784575966114299788842302085249695831517064950839407"
                    ]
                },
                {
                    "values": [
                        "1",
                        "0"
                    ]
                }
            ],
            "pi_c": [
                "11454890441803773990669541648478857444364496620561490584390018845816952078066",
                "11602134405069161074303940861377076534746484968292843878270724121787321720399",
                "1"
            ]
        },
        "ephemeral_key_pair": "6ee8dfb5119ac9915cb2a90de1be47f207a780cb09a3355981318a328991cd1e",
        "beneficiaries": [
            "0xfe39693d77c7c83e26ff7df39c13fc36f9cc88f5"
        ]
    });
    let AuthorizationData {
        salt,
        proof,
        ephemeral_key_pair,
        beneficiaries,
    } = serde_json::from_value::<AuthorizationData>(data).unwrap();

    let address_beneficiaries = beneficiaries
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
            &ephemeral_key_pair,
        )
        .await?,
    );
    // let login_data = LoginData {
    //     salt,
    //     proof,
    //     ephemeral_key_pair,
    //     beneficiaries,
    // };
    let jwt_options =
        JWTOptions::<LocalWallet>::try_init(token_data, ephemeral_key_pair, proof, salt)?;

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
