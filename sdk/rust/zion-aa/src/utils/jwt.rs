use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};

use crate::types::jwt::JWTPayload;

pub fn decode_jwt(token: &str) -> Result<TokenData<JWTPayload>, jsonwebtoken::errors::Error> {
    let header = decode_header(token).unwrap();
    let key = DecodingKey::from_rsa_der(&[]);

    let mut validation = Validation::new(header.alg);
    validation.insecure_disable_signature_validation();
    validation.validate_aud = false;

    decode::<JWTPayload>(token, &key, &validation)
}

#[cfg(test)]
mod test {
    use jsonwebtoken::decode_header;

    use super::decode_jwt;

    #[test]
    fn test_decode_jwt() {
        let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImlHS1VBT05tQ0RBUU5oQXVCNHFhOUtCaiIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MjIzMzQzNjcsImV4cCI6MTcyMjQyMDc2NywibmJmIjoxNzIyMzM0MzY3LCJpc3MiOiJodHRwczovL2lkLnRlbGVncmFtLm9yZyIsInN1YiI6IjcxMDk3NDA0ODIiLCJhdWQiOiI3MTA5NzQwNDgyIn0.XFDTx_hlLg4nT4rEaVLItqH_6TZ3PgnTmO7yiAtxN7rIEZEi0BlmkLw6M7dRsi7UXvtsvt49vrodlS0BkztpyK4Qs2BNWFckcUzjVoLLkiTRKG6j2QmKjqKidbJlf2N2vjEhhNh0__vd7BpEdyhqkl6qCLIsm-8MvOd1vEwZmfvMGAZyvGmeHLvVb911w50drlbUpn3yMiVAEoybfqCS20pYYfj3-oYg1tO0ZyUofgNZK0uBmMcvV7RYnNgKpGjW4JYtb-qLmY7Ly3EtM4lMiZFSWCv6JdxZETG9dQ35x7QTUxFPCAM_YjdAlNm8EgkSgHPSVptZeqLEE_M_usaT4Q";

        let header = decode_header(token).unwrap();
        let ret = decode_jwt(token).unwrap();

        println!("{:?}", ret);
    }
}
