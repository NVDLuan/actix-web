use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use rand::Rng;

const SECRET_KEY: &[u8] = b"your_secret_key";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn generate_access_token(username: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .expect("invalid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
        .expect("JWT encoding failed")
}

pub fn generate_refresh_token() -> String {
    let rng = rand::thread_rng();
    let refresh_token: String = rng
        .sample_iter(rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    refresh_token
}

