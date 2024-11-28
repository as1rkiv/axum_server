mod dept;
mod menu;
mod perm;
mod role;
mod user;

use crate::state::AppState;
use axum::Router;

// /api/v1/admin
pub fn routes(state: &AppState) -> Router {
    Router::new()
        .nest("/role", role::routes(state.clone()))
        .nest("/menu", menu::routes(state.clone()))
        .nest("/dept", dept::routes(state.clone()))
        .nest("/perm", perm::routes(state.clone()))
        .nest("/user", user::routes(state.clone()))
}
