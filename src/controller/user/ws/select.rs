use super::{WebSocketUserListResp, MIN_USER_CHAT_MESSAGE_RESPONSE};
use crate::{
    common::error::Error,
    model::sys::user::TABLE_SYS_USER,
    state::{
        user::UserState,
        websocket::{WebSocketMessage, TABLE_MESSAGE_RECORD},
        AppState,
    },
    store::mysql::QueryBuilderExt,
};
use serde::Deserialize;
use sqlx::QueryBuilder;
use validator::Validate;

/*
**  获取当前用户对话列表
*/

pub async fn query_user_dialog_list(
    app_state: &AppState,
    user_state: &UserState,
) -> Result<Vec<WebSocketUserListResp>, Error> {
    // 查询用户
    let query = format!(
        r#"SELECT u.id, u.fullname, u.avatar, MAX(mr.id) AS last_msg 
        FROM `{TABLE_SYS_USER}` u 
        LEFT JOIN `{TABLE_MESSAGE_RECORD}` mr 
        ON (mr.sender = u.id OR mr.receiver = u.id) 
        WHERE u.is_deleted = 0 AND u.id <> ? 
        GROUP BY u.id, u.fullname, u.avatar 
        ORDER BY last_msg DESC"#
    );
    let userlist: Vec<WebSocketUserListResp> = sqlx::query_as(&query)
        .bind(user_state.get_id())
        .fetch_all(app_state.mysql())
        .await?;

    Ok(userlist)
}

/*
**  拉取用户与某人的最近聊天记录
*/

#[derive(Debug, Deserialize, Validate)]
pub struct QueryChatHistory {
    #[validate(required(message = "参数无效"), range(min = 1, message = "参数无效"))]
    with: Option<i64>,
}
pub async fn query_history_by_user(
    app_state: &AppState,
    user_state: &UserState,
    params: QueryChatHistory,
) -> Result<Vec<WebSocketMessage>, Error> {
    // 解析参数
    let with_user = params.with.ok_or(Error::Params("参数无效"))?;

    // 查询未读消息的数量
    let unread_count_query = format!(
        r#"SELECT COUNT(*) FROM `{TABLE_MESSAGE_RECORD}` mr
        WHERE ( 
            (mr.sender = ? AND mr.receiver = ?) 
            OR 
            (mr.receiver = ? AND mr.sender = ?)
        ) AND mr.read = 0"#,
    );
    let unread_count: i64 = sqlx::query_scalar(&unread_count_query)
        .bind(user_state.get_id())
        .bind(with_user)
        .bind(user_state.get_id())
        .bind(with_user)
        .fetch_one(app_state.mysql())
        .await?;

    // 构建查询
    let mut query_builder = QueryBuilder::select("*", TABLE_MESSAGE_RECORD, "mr");

    // 构建查询
    query_builder
        .push(" WHERE ((mr.sender = ")
        .push_bind(user_state.get_id())
        .push(" AND mr.receiver = ")
        .push_bind(with_user)
        .push(") OR (mr.receiver = ")
        .push_bind(user_state.get_id())
        .push(" AND mr.sender = ")
        .push_bind(with_user)
        .push("))")
        .order_by("mr.id DESC");

    // 未读数量不满足最小返回数量
    if unread_count <= MIN_USER_CHAT_MESSAGE_RESPONSE {
        // 返回固定条数
        query_builder.limit(MIN_USER_CHAT_MESSAGE_RESPONSE);
    } else {
        // 返回从未读开始的所有消息
        query_builder.limit(unread_count);
    }

    let msgs: Vec<WebSocketMessage> = query_builder
        .build_query_as()
        .fetch_all(app_state.mysql())
        .await?;

    Ok(msgs)
}

/*
**  加载用户与某人的更多聊天记录
*/

#[derive(Debug, Deserialize, Validate)]
pub struct QueryMessageLoad {
    #[validate(required(message = "参数无效"), range(min = 1, message = "参数无效"))]
    with: Option<i64>,
    #[validate(length(min = 32, max = 32, message = "参数无效"))]
    ident: String,
}
pub async fn query_message_by_user_msg_id(
    app_state: &AppState,
    user_state: &UserState,
    params: QueryMessageLoad,
) -> Result<Vec<WebSocketMessage>, Error> {
    // 解析参数
    let with_user = params.with.ok_or(Error::Params("参数无效"))?;
    let ident = params.ident;

    // 构建查询
    let mut query_builder = QueryBuilder::select("*", TABLE_MESSAGE_RECORD, "mr");

    // 构建查询
    query_builder
        .push(" WHERE ((mr.sender = ")
        .push_bind(user_state.get_id())
        .push(" AND mr.receiver = ")
        .push_bind(with_user)
        .push(") OR (mr.receiver = ")
        .push_bind(user_state.get_id())
        .push(" AND mr.sender = ")
        .push_bind(with_user)
        .push(")) AND mr.id < ")
        .push(format!(
            "(SELECT id FROM `{TABLE_MESSAGE_RECORD}` WHERE ident = "
        ))
        .push_bind(ident)
        .push(")")
        .order_by("mr.id DESC")
        .limit(MIN_USER_CHAT_MESSAGE_RESPONSE);

    let msgs: Vec<WebSocketMessage> = query_builder
        .build_query_as()
        .fetch_all(app_state.mysql())
        .await?;

    Ok(msgs)
}
