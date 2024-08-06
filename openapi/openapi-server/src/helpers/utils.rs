use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::result::Result::Ok;
use tonic::{Response, Status};
use std::fs;
use std::path::Path;
use tokio::time::{sleep, Duration};

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