pub mod select;
pub mod update;
pub mod upload;

use crate::state::websocket::WebSocketMessage;
use serde::Serialize;
use sqlx::FromRow;

// 单用户聊天记录最小返回数量
const MIN_USER_CHAT_MESSAGE_RESPONSE: i64 = 10;

// 用户聊天列表响应
#[derive(Debug, Serialize, FromRow)]
pub struct WebSocketUserListResp {
    id: i64,

    // 姓名
    fullname: String,

    // 头像
    avatar: String,

    // 在线状态
    #[sqlx(skip)]
    msgs: Vec<WebSocketMessage>,
}
