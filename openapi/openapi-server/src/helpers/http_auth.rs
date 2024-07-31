use {
    actix_web::{dev::ServiceRequest, Error},
    chrono::Utc,
    jsonwebtoken::{
        decode, decode_header, errors::ErrorKind, Algorithm, DecodingKey, TokenData, Validation,
    },
    serde::{Deserialize, Serialize},
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
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(&[]);
    let header = match decode_header(token) {
        Ok(header) => header,
        Err(e) => {
            return Err(e);
        }
    };
    let algorithms = header.alg;
    let mut validation = Validation::new(algorithms);
    validation.insecure_disable_signature_validation();
    validation.validate_aud = false;
    // validation.set_audience(&["7109740482"]);
    // Decode the token without verifying the signature
    let token_data = decode::<Claims>(token, &key, &validation);

    match token_data {
        Ok(data) => {
            println!("data {:?}", data);
            let now = Utc::now();
            // Number of seconds since the Unix epoch
            let auth_date: u32 = now.timestamp() as u32;
            println!("auth_date {:?}", auth_date);
            if data.claims.exp > auth_date {
                println!("Token is valid");
            } else {
                return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken));
            }
            return Ok(data);
        }
        Err(err) => {
            println!("err {:?}", err);
            return Err(err);
        }
    }
}
