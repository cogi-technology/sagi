use {
    crate::{config::TelegramAuthConfig, entity::telegram::GetRequestType},
    actix_web::{dev::ServiceRequest, Error},
    jsonwebtoken::{decode_header, errors::ErrorKind, Algorithm},
    jwt_simple::{
        algorithms::{EdDSAPublicKeyLike, RSAPublicKeyLike},
        claims::{JWTClaims, NoCustomClaims},
    },
    reqwest::Client,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u32,
    uid: Option<String>,
}

pub async fn validator_token(
    req: ServiceRequest,
    credentials: actix_web_httpauth::extractors::bearer::BearerAuth,
    telegram_auth_config: TelegramAuthConfig,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    match validate_token(token, telegram_auth_config).await {
        Ok(_) => Ok(req),
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid Token"), req)),
    }
}

fn base64_url_to_base64(base64_url: &str) -> String {
    let base64 = base64_url.replace('-', "+").replace('_', "/");
    // Add padding
    let padding_len = (4 - base64.len() % 4) % 4;
    let base64_with_padding = format!("{}{}", base64, "=".repeat(padding_len));

    base64_with_padding
}

// fn is_file_exists(path: &PathFile) -> bool {
//     path.exists() && path.is_file()
// }

#[allow(deprecated)]
async fn validate_token(
    token: &str,
    telegram_auth_config: TelegramAuthConfig,
) -> Result<JWTClaims<NoCustomClaims>, jsonwebtoken::errors::Error> {
    let header = match decode_header(token) {
        Ok(header) => header,
        Err(e) => {
            return Err(e);
        }
    };
    let algorithms: Algorithm = header.alg;
    let kid_header = header.kid.unwrap();
    //
    // let key = DecodingKey::from_rsa_der(&[]);
    // let mut validation = Validation::new(header.alg);
    // validation.insecure_disable_signature_validation();
    // validation.validate_aud = false;
    // let payload = match decode::<Claims>(token, &key, &validation) {
    //     Ok(data) => data,
    //     Err(_) => return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))
    // };
    // let _= match payload.claims.uid {
    //     Some(session_uuid) => {
    //         let session_file: String = format!("sessions/session_{}", session_uuid.to_string());
    //         let path = PathFile::new(&session_file);

    //         if !is_file_exists(&path) {
    //             return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))
    //         }
    //     },
    //     None => return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))
    // };
    //
    let client = Client::new();
    let base_url = telegram_auth_config.next_public_server_login_author.clone();
    let url = format!("{}/auth/v1/oidc/certs", base_url);
    let cert = match client.get(&url).send().await {
        Ok(response) => response,
        Err(_) => return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken)),
    };
    let cert_data = match cert.json::<GetRequestType>().await {
        Ok(data) => data,
        Err(_) => return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken)),
    };

    for key in cert_data.keys {
        if key.kid.unwrap() == kid_header {
            let _n = key.n.unwrap();
            let _e = key.e.unwrap();
            let modulus = match base64::decode(base64_url_to_base64(&_n)) {
                Ok(modulus) => modulus,
                Err(_) => return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken)),
            };
            let exponent = match base64::decode(base64_url_to_base64(&_e)) {
                Ok(exponent) => exponent,
                Err(_) => return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken)),
            };
            match algorithms {
                Algorithm::RS256 => {
                    let key = jwt_simple::algorithms::RS256PublicKey::from_components(
                        &modulus, &exponent,
                    )
                    .map_err(|_| jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))?;
                    return match key.verify_token::<NoCustomClaims>(token, None) {
                        Ok(data) => Ok(data),
                        Err(_) => {
                            return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))
                        }
                    };
                }
                Algorithm::RS384 => {
                    let key = jwt_simple::algorithms::RS384PublicKey::from_components(
                        &modulus, &exponent,
                    )
                    .map_err(|_| jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))?;
                    return match key.verify_token::<NoCustomClaims>(token, None) {
                        Ok(data) => Ok(data),
                        Err(_) => {
                            return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))
                        }
                    };
                }
                Algorithm::RS512 => {
                    let key = jwt_simple::algorithms::RS512PublicKey::from_components(
                        &modulus, &exponent,
                    )
                    .map_err(|_| jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))?;
                    return match key.verify_token::<NoCustomClaims>(token, None) {
                        Ok(data) => Ok(data),
                        Err(_) => Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken)),
                    };
                }
                Algorithm::EdDSA => {
                    let _x = key.x.unwrap();
                    let x_plus = match base64::decode(base64_url_to_base64(&_x)) {
                        Ok(exponent) => exponent,
                        Err(_) => {
                            return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))
                        }
                    };
                    let key: jwt_simple::prelude::Ed25519PublicKey =
                        jwt_simple::algorithms::Ed25519PublicKey::from_der(&x_plus).map_err(
                            |_| jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken),
                        )?;
                    return match key.verify_token::<NoCustomClaims>(token, None) {
                        Ok(data) => Ok(data),
                        Err(_) => Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken)),
                    };
                }
                _default => {
                    return Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken));
                }
            };
        }
    }
    Err(jsonwebtoken::errors::Error::from(ErrorKind::InvalidToken))
}
