use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use sqlx::{
    decode::Decode,
    error::BoxDynError,
    mysql::{MySql, MySqlTypeInfo},
    Database, Type,
};
use std::fmt;

// 消息类型
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum WebSocketMessageType {
    Error = 0,
    Text = 1,
    Xml = 2,
    Img = 3,
    Video = 4,
    Voice = 5,
    Location = 6,
}

// 默认
impl Default for WebSocketMessageType {
    fn default() -> Self {
        Self::Error
    }
}

// 显示
impl fmt::Display for WebSocketMessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

// 序列化
impl Serialize for WebSocketMessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

// 反序列化
impl<'de> Deserialize<'de> for WebSocketMessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            1 => Ok(Self::Text),
            2 => Ok(Self::Xml),
            3 => Ok(Self::Img),
            4 => Ok(Self::Video),
            5 => Ok(Self::Voice),
            6 => Ok(Self::Location),
            _ => Ok(Self::default()),
        }
    }
}

// sqlx 识别 WebSocketMessageType
impl Type<MySql> for WebSocketMessageType {
    fn type_info() -> MySqlTypeInfo {
        <u8 as Type<MySql>>::type_info()
    }
}

// 数据库 TINYINT UNSIGNED 解码
impl<'r> Decode<'r, MySql> for WebSocketMessageType {
    fn decode(value: <MySql as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match <u8 as Decode<MySql>>::decode(value)? {
            1 => Ok(Self::Text),
            2 => Ok(Self::Xml),
            3 => Ok(Self::Img),
            4 => Ok(Self::Video),
            5 => Ok(Self::Voice),
            6 => Ok(Self::Location),
            _ => Ok(Self::default()),
        }
    }
}

// u8转消息类型
impl From<u8> for WebSocketMessageType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Text,
            2 => Self::Xml,
            3 => Self::Img,
            4 => Self::Video,
            5 => Self::Voice,
            6 => Self::Location,
            _ => Self::default(),
        }
    }
}
