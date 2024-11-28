use crate::{
    common::error::Error,
    middleware::jwt::{error::AuthError, token::verify},
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserState {
    uid: i64,
}

impl UserState {
    pub fn new(user_id: i64) -> Self {
        Self { uid: user_id }
    }

    pub fn get_id(&self) -> i64 {
        self.uid
    }

    pub fn is_admin(&self) -> bool {
        self.uid == 1
    }
}

// 从请求中自动提取 Authorization Header
// 验证用户 Token 是否合法
// 验证通过后 UserState 会自动放入请求中
#[async_trait]
impl<S> FromRequestParts<S> for UserState
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Error> {
        // 从 Headers 中获取 Authorization Token
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        // 解析 Token
        Ok(verify(bearer.token()).await?)
    }
}
