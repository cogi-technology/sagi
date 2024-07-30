use {
    actix_web::{dev::ServiceRequest, Error}, chrono::Utc, jsonwebtoken::{decode, decode_header, errors::ErrorKind, Algorithm, DecodingKey, Validation}, serde::{Deserialize, Serialize}
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u32,
}

pub async fn validator_token(
    req: ServiceRequest,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
    auth_secret: String,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    match validate_token(token, auth_secret.as_str()) {
        Ok(_) => Ok(req),
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid Token"), req)),
    }
}

fn validate_token(
    token: &str,
    secret: &str,
) -> Result<bool, jsonwebtoken::errors::Error> {
    let header = decode_header(token).unwrap();
    let algorithms = header.alg;
    match decode_header(token) {
        Ok(header) => {
            let algorithm = header.alg;
            if algorithm != Algorithm::RS256 {
                return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken));
            }
            return Ok(true);
        }
        Err(e) => {
            return Err(e);
        }
    }
}
