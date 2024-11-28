mod types;

use crate::common::error::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_vec};
use sqlx::FromRow;
use uuid::Uuid;

// 导出消息类型
pub use types::WebSocketMessageType;

/// 消息记录表名
pub const TABLE_MESSAGE_RECORD: &str = "msg_record";

// 消息体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WebSocketMessage {
    pub id: Option<i64>,
    // 会话ID
    pub session: Option<i64>,
    // 唯一标识
    pub ident: String,
    // 发送者
    pub sender: i64,
    // 接收者
    pub receiver: i64,
    // 消息类型
    pub msg_type: WebSocketMessageType,
    // 消息内容
    pub content: String,
    // 已读状态
    pub read: bool,
    // 发送IP
    pub sent_ip: Option<String>,
    // 发送时间
    pub sent_at: DateTime<Utc>,
}

#[allow(dead_code)]
impl WebSocketMessage {
    // 获取发送人
    pub fn get_sender(&self) -> i64 {
        self.sender
    }

    // 获取接收人
    pub fn get_receiver(&self) -> i64 {
        self.receiver
    }

    // 获取消息类型
    pub fn get_msg_type(&self) -> WebSocketMessageType {
        self.msg_type
    }

    // 获取消息内容
    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    // 获取发送时间
    pub fn get_sent_time(&self) -> DateTime<Utc> {
        self.sent_at
    }

    // 序列化到字节序列
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(to_vec(self)?)
    }

    // 从字节序列反序列化
    pub fn from_bytes(raw: &[u8]) -> Result<Self, Error> {
        Ok(from_slice(raw)?)
    }
}

// 元组转换完整消息
impl From<(i64, String, NewMessage)> for WebSocketMessage {
    fn from(value: (i64, String, NewMessage)) -> Self {
        Self {
            id: None,
            session: value.2.session,
            ident: Uuid::new_v4().as_simple().to_string(),
            sender: value.0,
            receiver: value.2.receiver,
            msg_type: value.2.msg_type,
            content: value.2.content,
            read: false,
            sent_ip: Some(value.1),
            sent_at: value.2.sent_at.unwrap_or(Utc::now()),
        }
    }
}

/// 新收到的消息
#[derive(Debug, Deserialize, Serialize)]
pub struct NewMessage {
    // 订单ID
    pub session: Option<i64>,
    // 接收者
    pub receiver: i64,
    // 消息类型
    pub msg_type: WebSocketMessageType,
    // 消息内容
    pub content: String,
    // 发送时间
    #[serde(skip)]
    pub sent_at: Option<DateTime<Utc>>,
}

impl NewMessage {
    pub fn new(content: Vec<u8>) -> Result<Self, Error> {
        // 从二进制 json 解析
        Ok(from_slice(&content)?)
    }
}
