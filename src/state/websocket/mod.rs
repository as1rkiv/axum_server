mod command;
mod connection;
mod manager;
mod message;

use crate::state::{user::UserState, AppState};
use axum::extract::ws::WebSocket;
use chrono::Utc;
use futures::stream::StreamExt;
use std::net::SocketAddr;

// 导出类型
pub use command::WebSocketCommand;
pub use connection::WebSocketConnection;
pub use manager::WebSocketManager;
pub use message::{NewMessage, WebSocketMessage, TABLE_MESSAGE_RECORD};

// 处理 WebSocket 连接
pub async fn new_connection(
    websocket: WebSocket,
    app_state: AppState,
    user_state: UserState,
    ipaddr: SocketAddr,
) {
    // 分离 socket
    let (sender, mut receiver) = websocket.split();

    // 添加新连接
    let conn_id = Utc::now().timestamp_millis();
    app_state
        .websocket()
        .add_conn(
            user_state.get_id(),
            conn_id,
            WebSocketConnection::new(sender),
        )
        .await
        .unwrap_or_else(|_| {
            return;
        });

    // 监听消息
    while let Some(msg) = receiver.next().await {
        if WebSocketConnection::proc_all_msg(msg, |msg| {
            app_state
                .websocket()
                .proc_conn(user_state.get_id(), msg, ipaddr)
        })
        .await
        .is_break()
        {
            break;
        }
    }

    // 移除断开连接的用户
    app_state
        .websocket()
        .remove_conn(user_state.get_id(), conn_id)
        .await
        .ok();
}
