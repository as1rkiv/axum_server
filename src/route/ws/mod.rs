use crate::{
    common::{
        error::Error,
        response::{
            json::{Notify, SingleList},
            Response,
        },
    },
    controller::user::ws::{
        select, update,
        upload::{self, UploadChatDataResp},
        WebSocketUserListResp,
    },
    state::{user::UserState, websocket::WebSocketMessage, AppState},
};
use axum::{
    extract::{Multipart, Query, State},
    routing::{get, post, put},
    Json, Router,
};
use validator::Validate;

// WebSocket 路由
pub fn routes(state: &AppState) -> Router {
    Router::new()
        .route("/dialog", get(get_user_list))
        .route("/history", get(get_user_chat_hostory))
        .route("/more", get(get_user_chat_message_more))
        .route("/file", post(upload_chat_file))
        .route("/read", put(update_user_chat_read))
        .with_state(state.clone())
}

/*
**  获取当前好友列表
*/

pub async fn get_user_list(
    State(app_state): State<AppState>,
    user_state: UserState,
) -> Result<SingleList<WebSocketUserListResp>, Error> {
    // 查询用户
    Ok(select::query_user_dialog_list(&app_state, &user_state)
        .await?
        .into())
}

/*
**  获取与某人的聊天记录
*/

pub async fn get_user_chat_hostory(
    State(app_state): State<AppState>,
    user_state: UserState,
    Query(params): Query<select::QueryChatHistory>,
) -> Result<SingleList<WebSocketMessage>, Error> {
    params.validate()?;
    Ok(
        select::query_history_by_user(&app_state, &user_state, params)
            .await?
            .into(),
    )
}

/*
**  翻页加载与某人的聊天
*/

pub async fn get_user_chat_message_more(
    State(app_state): State<AppState>,
    user_state: UserState,
    Query(params): Query<select::QueryMessageLoad>,
) -> Result<SingleList<WebSocketMessage>, Error> {
    params.validate()?;
    Ok(
        select::query_message_by_user_msg_id(&app_state, &user_state, params)
            .await?
            .into(),
    )
}

/*
**  标记已读某人消息
*/

pub async fn update_user_chat_read(
    State(app_state): State<AppState>,
    user_state: UserState,
    Json(payload): Json<update::UpdateChatRead>,
) -> Result<Notify, Error> {
    payload.validate()?;

    Ok(update::update_chat_read(&app_state, &user_state, payload)
        .await?
        .into())
}

/*
**  上传文件
*/

pub async fn upload_chat_file(multipart: Multipart) -> Result<Response<UploadChatDataResp>, Error> {
    Ok(upload::upload_chat_files(multipart).await?.into())
}
