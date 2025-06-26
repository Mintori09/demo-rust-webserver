use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::errors::{error_message::ErrorMessage, http_error::HttpError};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn generate_token(
    user_id: &str,
    secret: &[u8],
    expires_in_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
    }

    let iat = Utc::now().timestamp() as usize;
    let exp = (Utc::now() + Duration::minutes(expires_in_seconds)).timestamp() as usize;

    let claims = TokenClaims {
        sub: user_id.to_owned(),
        iat,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, HttpError> {
    decode::<TokenClaims>(
        &token.into(),
        &DecodingKey::from_secret(&secret),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map(|data| data.claims.sub)
    .map_err(|_| HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()))
}
