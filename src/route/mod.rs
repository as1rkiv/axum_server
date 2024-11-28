mod admin;
mod base;
mod index;
mod opt;
mod user;
mod ws;

use crate::{config, middleware::privilege, state::AppState};
use axum::{middleware::from_fn_with_state, routing::get_service, Router};
use tower_http::services::ServeDir;

// 根路由
pub async fn init(state: AppState) -> Router {
    // 静态服务
    let static_srv = static_service().await;

    Router::new()
        .nest("/", index_service())
        .nest("/api", api_service(&state))
        .nest_service("/static", get_service(static_srv))
}

// 主页服务
fn index_service() -> Router {
    Router::new().merge(index::routes())
}

// API 服务
fn api_service(state: &AppState) -> Router {
    Router::new()
        .nest("/admin", admin::routes(state))
        .nest("/user", user::routes(state))
        .nest("/ws", ws::routes(state))
        .nest("/opt", opt::routes(state))
        .layer(from_fn_with_state(state.clone(), privilege::enforcer))
        .merge(base::routes(state))
}

// 静态服务
async fn static_service() -> ServeDir {
    // 从配置中读取静态文件目录
    let conf = config::get_config().await;
    let dir = conf.server.get_static_dir();

    ServeDir::new(dir)
}
