use crate::{
    common::{
        error::Error,
        request::Pagination,
        response::json::{Notify, QuantityList},
    },
    controller::admin::menu::{
        delete, insert, select, update, InsertOrUpdatePermPayload, PermListResp,
    },
    state::{user::UserState, AppState},
};
use axum::{
    extract::{Json, Path, State},
    routing::{delete, get, post, put},
    Router,
};
use validator::Validate;

// 接口路由树
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_perm_list))
        .route("/", post(add_perm))
        .route("/:id", put(update_perm))
        .route("/:id", delete(delete_perm))
        .with_state(state)
}

// 接口列表
async fn get_perm_list(
    State(app_state): State<AppState>,
    params: Pagination<select::PermListQuery>,
) -> Result<QuantityList<PermListResp>, Error> {
    // 查询菜单
    Ok(select::query_perm_list(app_state.mysql(), params)
        .await?
        .into())
}

// 添加接口
async fn add_perm(
    State(app_state): State<AppState>,
    user_state: UserState,
    Json(payload): Json<InsertOrUpdatePermPayload>,
) -> Result<Notify, Error> {
    payload.validate()?;
    // 添加菜单
    Ok(
        insert::insert_perm(&app_state, user_state.get_id(), payload)
            .await?
            .into(),
    )
}

// 更新接口
async fn update_perm(
    State(app_state): State<AppState>,
    Path(menu_id): Path<i64>,
    _: UserState,
    Json(payload): Json<InsertOrUpdatePermPayload>,
) -> Result<Notify, Error> {
    payload.validate()?;
    Ok(update::update_perm_by_id(&app_state, menu_id, payload)
        .await?
        .into())
}

// 删除接口
async fn delete_perm(
    State(app_state): State<AppState>,
    Path(menu_id): Path<i64>,
    _: UserState,
) -> Result<Notify, Error> {
    Ok(delete::delete_menu_or_perm_by_id(&app_state, menu_id)
        .await?
        .into())
}
