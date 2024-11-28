use crate::{
    common::{
        error::Error,
        response::json::{Notify, SingleList},
    },
    controller::admin::{
        dept::{delete, insert, select, update, InsertOrUpdateDeptPayload},
        dept_menu::select::query_dept_menus,
        menu::options,
    },
    model::sys::dept::Dept,
    state::{user::UserState, AppState},
};
use axum::{
    extract::{Json, Path, Query, State},
    routing::{delete, get, post, put},
    Router,
};
use validator::Validate;

// 部门路由树
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_dept_tree_list))
        .route("/", post(add_dept))
        .route("/:id", get(get_dept_menus))
        .route("/:id", put(update_dept))
        .route("/:id", delete(delete_dept))
        .route("/privs", get(get_dept_privileges))
        .with_state(state)
}

// 查询部门树列表
async fn get_dept_tree_list(
    State(app_state): State<AppState>,
    Query(params): Query<select::DeptTreeQuery>,
) -> Result<SingleList<Dept>, Error> {
    params.validate()?;

    Ok(select::query_dept_tree(app_state.mysql(), params)
        .await?
        .into())
}

// 添加部门
async fn add_dept(
    State(app_state): State<AppState>,
    user_state: UserState,
    Json(payload): Json<InsertOrUpdateDeptPayload>,
) -> Result<Notify, Error> {
    payload.validate()?;
    Ok(
        insert::insert_dept(&app_state, user_state.get_id(), payload)
            .await?
            .into(),
    )
}

// 更新部门
async fn update_dept(
    State(app_state): State<AppState>,
    Path(dept_id): Path<i64>,
    _: UserState,
    Json(payload): Json<InsertOrUpdateDeptPayload>,
) -> Result<Notify, Error> {
    payload.validate()?;
    Ok(update::update_dept_by_id(&app_state, dept_id, payload)
        .await?
        .into())
}

// 删除部门
async fn delete_dept(
    State(app_state): State<AppState>,
    Path(dept_id): Path<i64>,
    _: UserState,
) -> Result<Notify, Error> {
    Ok(delete::delete_dept_by_id(&app_state, dept_id).await?.into())
}

// 查询部门可用权限
async fn get_dept_privileges(
    State(app_state): State<AppState>,
) -> Result<SingleList<options::PrivilegeOptions>, Error> {
    Ok(options::query_privilege_options(app_state.mysql())
        .await?
        .into())
}

// 查询部门当前权限
async fn get_dept_menus(
    State(app_state): State<AppState>,
    Path(dept_id): Path<i64>,
) -> Result<SingleList<i64>, Error> {
    Ok(query_dept_menus(app_state.mysql(), dept_id).await?.into())
}
