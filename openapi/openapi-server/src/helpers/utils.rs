use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Ok;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tonic::Status;

pub type Result<T> = std::result::Result<T, tonic::Status>;

pub fn into_anyhow(err: anyhow::Error) -> Status {
    Status::new(tonic::Code::Aborted, format!("{}", err))
}

async fn send_request<T: Serialize + DeserializeOwned>(
    client: &Client,
    method: Method,
    url: &str,
    body: Option<&T>,
    headers: Option<HashMap<String, String>>,
) -> std::result::Result<T, anyhow::Error> {
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

    let response = request.send().await?.json::<T>().await?;
    return Ok(response);
}
