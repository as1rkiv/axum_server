mod list;
mod notify;

use super::Response;
use crate::common::error::Error;
use serde::Serialize;

// 导出类型别名
pub type Notify = Response<notify::Notify>;
pub type QuantityList<T> = Response<list::QuantityList<T>>;
pub type SingleList<T> = Response<list::SingleList<T>>;

// Json 响应
#[derive(Debug, Serialize)]
pub struct JsonData<T: Serialize> {
    // 错误码
    pub code: i32,
    // 错误信息
    pub msg: String,
    // 返回数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

// 正常响应
impl<T: Serialize> JsonData<T> {
    pub fn new(code: i32, msg: impl ToString, data: Option<T>) -> Self {
        Self {
            code,
            msg: msg.to_string(),
            data,
        }
    }

    pub fn value(data: T) -> Self {
        Self::new(Error::Ok.to_code(), Error::Ok.to_string(), Some(data))
    }
}

// 空值响应
impl JsonData<()> {
    pub fn empty() -> Self {
        Self::new(Error::Ok.to_code(), Error::Ok.to_string(), None)
    }

    pub fn err(err: Error) -> Self {
        Self::new(err.to_code(), &err.to_string(), None)
    }
}

// json 转响应
impl<T: Serialize> From<JsonData<T>> for Response<T> {
    fn from(data: JsonData<T>) -> Self {
        Self::Json(data).into()
    }
}
