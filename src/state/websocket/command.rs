use super::{WebSocketConnection, WebSocketMessage};

// WebSocket MPSC 命令枚举
#[derive(Debug)]
pub enum WebSocketCommand {
    // 添加连接
    ConnAdd {
        user_id: i64,
        conn_id: i64,
        conn: WebSocketConnection,
    },

    // 移除连接
    ConnRemove {
        user_id: i64,
        conn_id: i64,
    },

    // 消息处理
    MsgProc {
        wsm: WebSocketMessage,
    },
}
