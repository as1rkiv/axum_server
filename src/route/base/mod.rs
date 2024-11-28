mod connect;
mod login;

use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

// 用户路由
pub fn routes(state: &AppState) -> Router {
    Router::new()
        .route("/login", post(login::post_user_login))
        .route("/connect", get(connect::ws_connect))
        .with_state(state.clone())
}
