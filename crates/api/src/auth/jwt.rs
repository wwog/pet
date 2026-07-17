use domain::user::Role;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: Uuid, role: Role) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::hours(2)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.as_str().to_owned(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}