use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    pub iss: String,
    pub exp: usize,
}

pub struct Jwt<'a> {
    pub secret: String,
    pub exp: i64,
    pub iss: &'a str,
}

impl<'a> Jwt<'a> {
    pub fn new(secret: String, exp: i64, iss: &'a str) -> Self {
        Self { secret, exp, iss }
    }
    pub fn new_claims(&self, id: i32, email: String) -> Claims {
        Claims {
            id,
            email,
            iss: self.iss.to_string(),
            exp: self.calc_claims_exp(),
        }
    }
    pub fn new_claims_with(&self, claims: &'a Claims) -> Claims {
        self.new_claims(claims.id, claims.email.clone())
    }

    fn calc_claims_exp(&self) -> usize {
        (Utc::now() + Duration::seconds(self.exp)).timestamp_millis() as usize
    }
    fn secret_bytes(&self) -> &[u8] {
        (&self.secret).as_bytes()
    }
    pub fn token(&self, claims: &'a Claims) -> Result<String, String> {
        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(self.secret_bytes()),
        )
        .map_err(|err| err.to_string())
    }
    pub fn verify_and_get(&self, token: &'a str) -> Result<Claims, String> {
        let mut v = Validation::new(jsonwebtoken::Algorithm::HS256);
        v.set_issuer(&[self.iss]);
        let token_data = decode(token, &DecodingKey::from_secret(self.secret_bytes()), &v)
            .map_err(|err| err.to_string())?;
        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const SECRET: &str = "axum.rs";
    const ISS: &str = "axum.rs";
    #[test]
    fn test_gen_token() {
        let jwt = Jwt::new(SECRET.to_string(), 120, ISS);
        let claims = jwt.new_claims(1, "team@axum.rs".to_string());
        let token = jwt.token(&claims).unwrap();
        println!("{:?}", token);
    }
    #[test]
    fn test_get_claims() {
        let jwt = Jwt::new(SECRET.to_string(), 120, ISS);
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwiZW1haWwiOiJ0ZWFtQGF4dW0ucnMiLCJpc3MiOiJheHVtLnJzIiwiZXhwIjoxNjYzOTIyMDE1MDc2fQ.yOLD0aus03jTOqTWUZdBDoxSeBhUQUZpRL8_-ZcYM84";
        let claims = jwt.verify_and_get(token).unwrap();
        println!("{:?}", claims);
    }
}
