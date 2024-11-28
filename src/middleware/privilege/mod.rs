pub mod access;
pub mod cache;
pub mod data;

use crate::{
    common::error::Error,
    state::{user::UserState, AppState},
};
use axum::{
    body::Body,
    extract::{OriginalUri, Request, State},
    middleware::Next,
    response::Response,
};

/// 路由访问权限验证
/// req.uri().path() 只能提取到当前层级
/// 使用 OriginalUri 获取原始 uri
pub async fn enforcer(
    OriginalUri(original_uri): OriginalUri,
    State(app_state): State<AppState>,
    user_state: Result<UserState, Error>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Error> {
    let user = user_state?;

    let resource = original_uri.path();
    let action = req.method().as_str();

    // 检查是否为 admin 或是否在免验证数组中
    if user.is_admin() {
        // 放行
        return Ok(next.run(req).await);
    }

    // 验证权限
    access::check(&app_state, &user, resource, action).await?;

    // 通过
    Ok(next.run(req).await)
}
