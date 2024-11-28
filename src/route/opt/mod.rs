mod car;
mod region;

use crate::state::AppState;
use axum::Router;

// 选项路由
pub fn routes(state: &AppState) -> Router {
    Router::new()
        .nest("/car", car::routes(state))
        .nest("/region", region::routes(state.clone()))
}
