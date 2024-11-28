use crate::{
    common::{
        error::Error,
        response::{
            json::{Notify, SingleList},
            Response,
        },
    },
    controller::user::{route, select, update, UserInfoResp},
    state::{user::UserState, AppState},
};
use axum::{
    extract::{Json, State},
    routing::{get, put},
    Router,
};
use serde::Deserialize;
use validator::Validate;

// 用户路由
pub fn routes(state: &AppState) -> Router {
    Router::new()
        .route("/info", get(get_user_info))
        .route("/menu", get(get_user_menu))
        .route("/password", put(put_user_password))
        .with_state(state.clone())
}

/*
**  用户信息
*/

async fn get_user_info(
    State(app_state): State<AppState>,
    user_state: UserState,
) -> Result<Response<UserInfoResp>, Error> {
    // 查询用户数据
    if let Some(info) =
        select::query_user_info_by_userid(app_state.mysql(), user_state.get_id()).await?
    {
        // 返回结果
        return Ok(info.into());
    }
    Err(Error::Auth("请重新登录"))
}

/*
**  用户菜单
*/

async fn get_user_menu(
    State(app_state): State<AppState>,
    user_state: UserState,
) -> Result<SingleList<route::MenuRoute>, Error> {
    // 根据权限判断菜单
    Ok(route::fetch_menus_by_user(&app_state, &user_state)
        .await?
        .into())
}

/*
**  修改密码
*/

#[derive(Debug, Deserialize, Validate)]
struct PasswordPayload {
    #[validate(length(min = 1, message = "参数无效"))]
    old: String,
    #[validate(length(min = 1, message = "参数无效"))]
    new: String,
}

async fn put_user_password(
    State(app_state): State<AppState>,
    user_state: UserState,
    Json(payload): Json<PasswordPayload>,
) -> Result<Notify, Error> {
    // 校验数据
    payload.validate()?;

    // 查询用户数据
    if let Some(user) = select::query_user_by_userid(app_state.mysql(), user_state.get_id()).await?
    {
        // 校验条件
        if !user.is_active {
            return Err(Error::Tips("账号已禁用".into()));
        }

        if user.password != payload.old {
            return Err(Error::Tips("当前密码错误".into()));
        }

        // 更新密码
        return Ok(update::update_user_password_by_userid(
            app_state.mysql(),
            user_state.get_id(),
            payload.new,
        )
        .await?
        .into());
    }

    Err(Error::Auth("请重新登录"))
}
