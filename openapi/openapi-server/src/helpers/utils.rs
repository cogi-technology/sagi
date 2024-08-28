use {
    crate::{
        config::TelegramAuthConfig,
        error::{into_anyhow, Result},
        services::zionauthorization::get_data_request_for_zion_logic,
    },
    anyhow::{Result as AnyhowResult, anyhow},
    ethers::{signers::LocalWallet, types::Address},
    jsonwebtoken::TokenData,
    reqwest::{Client, Method, RequestBuilder},
    serde::{de::DeserializeOwned, Serialize},
    std::{collections::HashMap, result::Result::Ok, sync::Arc},
    tonic::{metadata::MetadataMap, Response},
    zion_aa::{
        constants::{get_contract_wallet_operator, Networkish},
        contract_wallet::{
            client::{Client as ZionClient, ClientMethods},
            operator::Operator,
            wallet::ContractWallet,
        },
        types::{jwt::{JWTOptions, JWTPayload}, request::AuthorizationData},
    },
};

#[allow(dead_code)]
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

    match response.json::<T>().await {
        Ok(response) => Ok(Response::new(response)),
        Err(e) => Err(into_anyhow(e.into())),
    }
}

// pub async fn send_request_text<U>(
//     client: &Client,
//     method: Method,
//     url: &str,
//     body: Option<&U>,
//     headers: Option<HashMap<String, String>>,
// ) -> Result<Response<String>>
// where
//     U: Serialize,
// {
//     let mut request: RequestBuilder = client.request(method, url);

//     // Set the body if provided
//     if let Some(b) = body {
//         request = request.json(b);
//     }

//     // Set headers if provided
//     if let Some(h) = headers {
//         for (key, value) in h {
//             request = request.header(&key, &value);
//         }
//     }

//     let response = match request.send().await {
//         Ok(response) => response,
//         Err(e) => return Err(into_anyhow(e.into())),
//     };

//     match response.text().await {
//         Ok(response) => Ok(Response::new(response)),
//         Err(e) => Err(into_anyhow(e.into())),
//     }
// }

pub async fn init_contract_wallet(
    header_metadata: &tonic::metadata::MetadataMap,
    rpc_endpoint: &str,
    telegram_auth_config: &TelegramAuthConfig,
) -> AnyhowResult<ContractWallet<ZionClient, LocalWallet>> {
    let (
        AuthorizationData {
            salt,
            proof,
            ephemeral_key_pair,
            beneficiaries,
        },
        token_data,
    ) = get_data_request_for_zion_logic(header_metadata, telegram_auth_config).await?;

    let address_beneficiaries = beneficiaries
        .iter()
        .map(|b| b.parse::<Address>().unwrap())
        .collect::<Vec<_>>();

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
    let jwt_options = JWTOptions::<LocalWallet>::try_init(
        token_data.clone(),
        ephemeral_key_pair,
        proof,
        salt.clone(),
    )?;

    let operator = Arc::new(Operator::new(
        contract_wallet_operator,
        Arc::clone(&client),
        address_beneficiaries,
    ));
    let TokenData {
        header: _,
        claims: payload,
    } = token_data;

    let contract_address = operator
        .get_address(payload.sub, salt, payload.iss, payload.aud)
        .await?;

    let mut contract_wallet = ContractWallet::<ZionClient, _>::new(contract_address, operator);
    contract_wallet.set_jwt(jwt_options);

    Ok(contract_wallet)
}


pub async fn get_payload_from_jwt(
    metadata: &MetadataMap,
) -> AnyhowResult<TokenData<JWTPayload>> {
    // Access a specific header, e.g., "authorization"
    let authorization_header = metadata
        .get("authorization")
        .ok_or(anyhow!("Authorization header not found"))?
        .to_str()?;
    if !authorization_header.starts_with("Bearer ") {
        return Err(anyhow!("Invalid authorization header"));
    }

    // Extract the JWT token by removing the "Bearer " prefix
    let token = &authorization_header["Bearer ".len()..];
    let parsed_token = zion_aa::utils::decode_jwt(token)?;

    Ok( parsed_token)
}

// pub async fn delete_file_after_time(file_path: &str, time: u64) -> bool {
//     // Sleep for time
//     // sleep(Duration::from_secs(24 * 60 * 60)).await;
//     sleep(Duration::from_secs(time)).await;
//     // Check if the file exists
//     if Path::new(file_path).exists() {
//         match fs::remove_file(file_path) {
//             Ok(res) => res,
//             Err(_) => {},
//         }
//     }
//     return true;
// }
