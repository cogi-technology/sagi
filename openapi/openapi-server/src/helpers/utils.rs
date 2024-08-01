use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Result as AnyhowResult;
use ethers::{signers::LocalWallet, types::Address};
use openapi_proto::zionauthorization_service::GetDataRequestForZionResponse;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
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
) -> AnyhowResult<ContractWallet<ZionClient, LocalWallet>> {
    let (
        GetDataRequestForZionResponse {
            salt,
            proof,
            ephemeral_key_pair,
            beneficiaries,
        },
        token_data,
    ) = get_data_request_for_zion_logic(header_metadata).await?;

    let address_beneficiaries = beneficiaries
        .iter()
        .map(|b| b.parse::<Address>().unwrap())
        .collect::<Vec<_>>();
    let proof = sdk_proofpoint_from(proof.unwrap());

    let contract_wallet_operator =
        get_contract_wallet_operator(Some(Networkish::Name("ziontestnet".into())));

    let rpc_endpoint = "https://devnet-rpc.zionx.network";
    let client = Arc::new(
        ZionClient::try_new(
            rpc_endpoint,
            contract_wallet_operator.chain_id,
            &ephemeral_key_pair,
        )
        .await?,
    );
    let login_data = LoginData {
        salt,
        proof,
        ephemeral_key_pair,
        beneficiaries,
    };
    let jwt_options = JWTOptions::<LocalWallet>::try_init(token_data, login_data)?;

    let operator = Arc::new(Operator::new(
        contract_wallet_operator,
        Arc::clone(&client),
        address_beneficiaries,
    ));
    let contract_address = "0x4307E9f6cEd7aC3deC02dD90040F45034d55F8ab".parse::<Address>()?;

    let mut contract_wallet = ContractWallet::<ZionClient, _>::new(contract_address, operator);
    contract_wallet.set_jwt(jwt_options);

    Ok(contract_wallet)
}
