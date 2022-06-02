use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    http, TypedHeader,
};
use jsonwebtoken as jwt;
use jsonwebtoken::Validation;
use serde::{Deserialize, Serialize};

const SECRET: &[u8] = b"deadbeef";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub username: String,
    exp: i64,
}

impl Claims {
    pub fn new(id: i32, username: String) -> Self {
        Claims {
            id,
            username,
            exp: get_epoch() + 14 * 24 * 60 * 60,
        }
    }

    pub fn generate(&self) -> String {
        let key = jwt::EncodingKey::from_secret(SECRET);
        let token = jwt::encode(&jwt::Header::default(), &self, &key).unwrap();
        token
    }
}

fn get_epoch() -> i64 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = http::StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .unwrap();

        let key = jwt::DecodingKey::from_secret(SECRET);
        let token = jwt::decode::<Claims>(bearer.token(), &key, &Validation::default()).unwrap();

        Ok(token.claims)
    }
}
