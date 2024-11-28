mod from;

use crate::common::response::{json::JsonData, Response};
use thiserror::Error;

// 错误类型
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("ok")]
    Ok,

    #[error("请求超时")]
    TimedOut,

    #[error("资源不存在")]
    NotFound,

    #[error("无访问权限")]
    PermissionDenied,

    #[error("当前不可用")]
    Unavailable,

    #[error("验证失败: {0}")]
    Auth(&'static str),

    #[error("参数错误: {0}")]
    Params(&'static str),

    #[error("{0}")]
    Tips(String),

    #[error("{0}")]
    Rejection(String),
}

impl Error {
    pub(super) fn to_code(&self) -> i32 {
        match self {
            Error::Ok => 0,
            Error::Auth(_) => 50014,
            _ => -1,
        }
    }
}

// 错误转响应
impl From<Error> for Response<()> {
    fn from(error: Error) -> Response<()> {
        match error {
            Error::Rejection(err) => Self::PlainText(err),
            _ => Self::Json(JsonData::err(error)),
        }
    }
}

// 错误转axum响应
impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Response::from(self).into_response()
    }
}
