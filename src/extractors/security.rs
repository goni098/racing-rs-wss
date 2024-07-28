use crate::shared::read_env;
use crate::{error::Error, telegram};
use axum::RequestPartsExt;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct MiniappUser {
    pub id: u64,
    #[serde(default = "is_premium_default")]
    pub is_premium: bool,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo_url: Option<String>,
}

fn is_premium_default() -> bool {
    false
}

pub struct WsAuth(pub MiniappUser);

#[async_trait]
impl<S> FromRequestParts<S> for WsAuth
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let bearer = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let init_data = bearer.token();

        let bot_token = read_env("TELOXIDE_TOKEN")?;

        telegram::validate::verify_init_data(init_data, &bot_token).map(Self)
    }
}
