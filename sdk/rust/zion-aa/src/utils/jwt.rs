use jsonwebtoken::{decode, decode_header, DecodingKey, TokenData, Validation};

use crate::types::jwt::JWTPayload;

pub fn decode_jwt(token: &str) -> Result<TokenData<JWTPayload>, jsonwebtoken::errors::Error> {
    let header = decode_header(token)?;
    let key = DecodingKey::from_rsa_der(&[]);

    let mut validation = Validation::new(header.alg);
    validation.insecure_disable_signature_validation();
    validation.validate_aud = false;

    decode::<JWTPayload>(token, &key, &validation)
}

#[cfg(test)]
mod test {
    use super::decode_jwt;

    #[test]
    fn test_decode_jwt() {
        let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImlHS1VBT05tQ0RBUU5oQXVCNHFhOUtCaiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MjIzMzE2NzYsImV4cCI6MTcyMjQxODA3NiwibmJmIjoxNzIyMzk0MjAyLCJpc3MiOiJodHRwczovL2lkLnRlbGVncmFtLm9yZyIsInN1YiI6IjkyNTMzOTg4MyIsImF1ZCI6IjcxMDk3NDA0ODIifQ.JdW3BZGqk5tY52Z0SOk2gZBbCvWZpxZGEQAhN22zFh7WO5GWXaQItEQOVNx_joBU6fBUO1EYFVN3Em720oEv2FujFJa1lW8v7V6h9QkuWb-da3r7_zTibLIa0w8VMZDywVUo79nWapiWeerHp0hC_P4m094IMzcT50u0n1-mmvQ9yYzgxH81qxs8keoaVs4YMo0Jt2Rvh4i7wlIiPGhKko0qyfDvHlYHF_epGtozRCkCznGb1aKxCbjzVSEPKzOJ6gJLiQ6vHHMGaS85bTBWHFGcIcBXpcUnfZi7ndb5FA4-byBbZEHsbd3bNjgDoS0TsbQTYE_QhRalwWPbXMv6kw";

        let ret = decode_jwt(token).unwrap();

        println!("{:?}", ret);
    }
}
