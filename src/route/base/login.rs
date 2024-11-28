use crate::{
    common::{error::Error, response::Response},
    controller::user::{select, update},
    middleware::jwt,
    state::{user::UserState, AppState},
};
use axum::extract::{ConnectInfo, Extension, Json, State};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use validator::Validate;

// 登陆请求
#[derive(Debug, Deserialize, Validate)]
pub(super) struct LoginPayload {
    #[validate(length(min = 1, message = "参数无效"))]
    username: String,
    #[validate(length(min = 1, message = "参数无效"))]
    password: String,
}

// 登陆响应
#[derive(Debug, Serialize)]
pub(super) struct LoginResponse {
    token: String,
}

// 用户登录
pub(super) async fn post_user_login(
    State(app_state): State<AppState>,
    Extension(ConnectInfo(addr)): Extension<ConnectInfo<SocketAddr>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Response<LoginResponse>, Error> {
    // 校验数据
    payload.validate()?;

    // 查询数据库
    let user = match select::query_user_by_username(app_state.mysql(), &payload.username).await? {
        Some(user) => user,
        None => return Err(Error::Tips("用户未注册".into())),
    };

    // 校验密码
    if user.password != payload.password {
        return Err(Error::Tips("密码错误".into()));
    }

    // 检查状态
    if !user.is_active {
        return Err(Error::Tips("账号禁用".into()));
    }

    // 更新登录信息 - 忽略错误
    update::update_user_last_login_by_userid(
        app_state.mysql(),
        user.id,
        addr.ip().to_string(),
        chrono::Utc::now(),
    )
    .await
    .ok();

    // 签发 Token
    Ok(LoginResponse {
        token: jwt::token::generate(UserState::new(user.id)).await?,
    }
    .into())
}
