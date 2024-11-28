use super::{claims::Claims, error::AuthError, secret::get_secret};
use crate::{common::error::Error, state::user::UserState};
use jsonwebtoken::{decode, encode, Header, Validation};

// 签发 Token
pub async fn generate(user: UserState) -> Result<String, Error> {
    // 填充信息
    let claims = Claims::new(user).await?;

    let encoding = &get_secret().await.encoding;

    // 生成 Token
    let token =
        encode(&Header::default(), &claims, encoding).map_err(|_| AuthError::TokenCreation)?;

    Ok(token)
}

// 验证 Token
pub async fn verify(token: &str) -> Result<UserState, Error> {
    let decoding = &get_secret().await.decoding;

    // 解析 Token
    let token_data = decode::<Claims>(token, decoding, &Validation::default())
        .map_err(|_| AuthError::InvalidToken)?;

    Ok(token_data.claims.state)
}
