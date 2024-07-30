use tonic::Status;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, tonic::Status>;

pub fn into_anyhow(err: anyhow::Error) -> Status {
    Status::new(tonic::Code::Aborted, format!("{}", err))
}

// pub async fn validator(
//     req: ServiceRequest,
//     credentials: BearerAuth,
//     secret: Arc<String>,
// ) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
//     if credentials.token() == "1" {
//         return Ok(req)
//     } else {
//         // return Err((AuthenticationError::from(config).into(), req));
//         return Err((actix_web::error::ErrorUnauthorized("Unauthorized"), req));
//     }
// }
