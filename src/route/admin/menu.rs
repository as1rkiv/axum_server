use crate::{
    common::{
        error::Error,
        response::json::{Notify, SingleList},
    },
    controller::admin::menu::{delete, insert, options, select, update, InsertOrUpdateMenuPayload},
    model::sys::menu::Menu,
    state::{user::UserState, AppState},
};
use axum::{
    extract::{Json, Path, Query, State},
    routing::{delete, get, post, put},
    Router,
};
use validator::Validate;

// 菜单路由树
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_menu_tree_list))
        .route("/", post(add_menu))
        .route("/:id", put(update_menu))
        .route("/:id", delete(delete_menu))
        .route("/options", get(get_menu_options))
        .with_state(state)
}

// 菜单联级选项
async fn get_menu_options(
    State(app_state): State<AppState>,
) -> Result<SingleList<options::MenuOptions>, Error> {
    Ok(options::query_menu_options(app_state.mysql()).await?.into())
}

// 菜单树列表
async fn get_menu_tree_list(
    State(app_state): State<AppState>,
    Query(params): Query<select::MenuTreeListQuery>,
) -> Result<SingleList<Menu>, Error> {
    // 查询菜单
    Ok(select::query_menu_tree_list(app_state.mysql(), params)
        .await?
        .into())
}

// 添加菜单
async fn add_menu(
    State(app_state): State<AppState>,
    user_state: UserState,
    Json(payload): Json<InsertOrUpdateMenuPayload>,
) -> Result<Notify, Error> {
    payload.validate()?;
    // 添加菜单
    Ok(
        insert::insert_menu(&app_state, user_state.get_id(), payload)
            .await?
            .into(),
    )
}

// 更新部门
async fn update_menu(
    State(app_state): State<AppState>,
    Path(menu_id): Path<i64>,
    _: UserState,
    Json(payload): Json<InsertOrUpdateMenuPayload>,
) -> Result<Notify, Error> {
    payload.validate()?;
    Ok(update::update_menu_by_id(&app_state, menu_id, payload)
        .await?
        .into())
}

// 删除部门
async fn delete_menu(
    State(app_state): State<AppState>,
    Path(menu_id): Path<i64>,
    _: UserState,
) -> Result<Notify, Error> {
    Ok(delete::delete_menu_or_perm_by_id(&app_state, menu_id)
        .await?
        .into())
}
