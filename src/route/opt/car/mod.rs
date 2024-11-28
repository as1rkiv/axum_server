mod brand;
mod series;

use crate::state::AppState;
use axum::Router;

// 车辆路由
pub fn routes(state: &AppState) -> Router {
    Router::new()
        .nest("/brand", brand::routes(state.clone()))
        .nest("/series", series::routes(state.clone()))
}
