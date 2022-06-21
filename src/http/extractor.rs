use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    Extension, TypedHeader,
};
use jsonwebtoken::Validation;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tracing::debug;

use super::{error::Error, ApiContext};

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);

#[derive(Serialize, Deserialize)]
struct Claims {
    pub user_id: String,
    pub exp: usize,
}

pub struct AuthUser {
    pub user_id: String,
}

impl AuthUser {
    pub(in crate::http) fn to_jwt(&self, ctx: &ApiContext) -> String {
        let key = jsonwebtoken::EncodingKey::from_secret(ctx.config.hmac_key.as_bytes());
        let claims = Claims {
            user_id: self.user_id.clone(),
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp() as usize,
        };
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &key)
            .expect("HMAC signing should be infallible");
        token
    }
}

#[async_trait]
impl<B> FromRequest<B> for AuthUser
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let ctx: Extension<ApiContext> = Extension::from_request(req)
            .await
            .expect("BUG: ApiContext was not added as an extension");

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|e| {
                    debug!("Authorization header is using the wrong scheme: {:?}", e);
                    Error::Unauthorized
                })?;

        let key = jsonwebtoken::DecodingKey::from_secret(ctx.config.hmac_key.as_bytes());

        let token = jsonwebtoken::decode::<Claims>(bearer.token(), &key, &Validation::default())
            .map_err(|e| {
                debug!("failed to parse Authorization header: {:?}", e);
                Error::Unauthorized
            })?;

        Ok(AuthUser {
            user_id: token.claims.user_id,
        })
    }
}
