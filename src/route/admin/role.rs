use crate::{
    common::{
        error::Error,
        request::Pagination,
        response::json::{Notify, QuantityList},
    },
    controller::admin::role::{delete, insert, select, update, InsertOrUpdateRolePayload},
    model::sys::role::Role,
    state::{user::UserState, AppState},
};
use axum::{
    extract::{Json, Path, State},
    routing::{delete, get, post, put},
    Router,
};
use validator::Validate;

// 角色路由树
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_role_card))
        .route("/", post(add_role))
        .route("/:id", put(update_role))
        .route("/:id", delete(delete_role))
        .with_state(state)
}

// 角色卡片列表
async fn get_role_card(
    State(app_state): State<AppState>,
    params: Pagination<select::RoleCardQuery>,
) -> Result<QuantityList<Role>, Error> {
    // 返回结果
    Ok(select::query_role_card(app_state.mysql(), params)
        .await?
        .into())
}

// 添加角色
async fn add_role(
    State(app_state): State<AppState>,
    user_state: UserState,
    Json(payload): Json<InsertOrUpdateRolePayload>,
) -> Result<Notify, Error> {
    payload.validate()?;
    Ok(
        insert::insert_role(&app_state, user_state.get_id(), payload)
            .await?
            .into(),
    )
}

// 更新角色
async fn update_role(
    State(app_state): State<AppState>,
    Path(role_id): Path<i64>,
    _: UserState,
    Json(payload): Json<InsertOrUpdateRolePayload>,
) -> Result<Notify, Error> {
    payload.validate()?;
    Ok(update::update_role_by_id(&app_state, role_id, payload)
        .await?
        .into())
}

// 删除角色
async fn delete_role(
    State(app_state): State<AppState>,
    Path(role_id): Path<i64>,
    _: UserState,
) -> Result<Notify, Error> {
    Ok(delete::delete_role_by_id(&app_state, role_id).await?.into())
}
