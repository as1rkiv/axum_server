use crate::{
    common::{
        error::Error,
        request::Pagination,
        response::json::{Notify, QuantityList, SingleList},
    },
    controller::admin::{
        dept::options::{query_dept_options, DeptOptions},
        role::options::{query_role_options, RoleOptions},
        user::{delete, insert, select, update, InsertOrUpdateUserPayload, UserListResp},
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
        .route("/", get(get_user_list))
        .route("/", post(add_user))
        .route("/:id", put(update_user))
        .route("/:id", delete(delete_user))
        .route("/dept", get(get_dept_options))
        .route("/role", get(get_role_options))
        .with_state(state)
}

// 用户部门树
async fn get_dept_options(
    State(app_state): State<AppState>,
) -> Result<SingleList<DeptOptions>, Error> {
    Ok(query_dept_options(app_state.mysql()).await?.into())
}

// 用户角色树
async fn get_role_options(
    State(app_state): State<AppState>,
) -> Result<SingleList<RoleOptions>, Error> {
    Ok(query_role_options(app_state.mysql()).await?.into())
}

// 用户列表
async fn get_user_list(
    State(app_state): State<AppState>,
    params: Pagination<select::UserListQuery>,
) -> Result<QuantityList<UserListResp>, Error> {
    // 查询菜单
    Ok(select::query_user_list(app_state.mysql(), params)
        .await?
        .into())
}

// 添加用户
async fn add_user(
    State(app_state): State<AppState>,
    user_state: UserState,
    Json(payload): Json<InsertOrUpdateUserPayload>,
) -> Result<Notify, Error> {
    payload.validate()?;
    Ok(
        insert::insert_user(&app_state, user_state.get_id(), payload)
            .await?
            .into(),
    )
}

// 更新用户
async fn update_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<i64>,
    _: UserState,
    Json(payload): Json<InsertOrUpdateUserPayload>,
) -> Result<Notify, Error> {
    payload.validate()?;
    Ok(update::update_user_by_id(&app_state, user_id, payload)
        .await?
        .into())
}

// 删除用户
async fn delete_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<i64>,
    _: UserState,
) -> Result<Notify, Error> {
    Ok(delete::delete_user_by_id(&app_state, user_id).await?.into())
}
