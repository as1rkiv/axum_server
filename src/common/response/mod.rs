#![allow(dead_code)]

mod constants;
pub mod json;

use axum::http::{header::CONTENT_TYPE, HeaderValue, StatusCode};
use serde::Serialize;

// 响应类型
#[derive(Debug, Serialize)]
pub enum Response<T: Serialize> {
    Json(json::JsonData<T>),
    Xml(String),
    PlainText(String),
}

// 空值响应
impl Response<()> {
    pub fn ok() -> Self {
        Self::Json(json::JsonData::empty())
    }
}

// 结构体转换json
impl<T: Serialize> From<T> for Response<T> {
    fn from(value: T) -> Self {
        Self::Json(json::JsonData::value(value))
    }
}

// 为 Response 实现 axum 响应
impl<T: Serialize> axum::response::IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        // 构建 Headers
        let mut headers = axum::http::HeaderMap::new();

        // 匹配 Response 枚举
        match self {
            Response::Json(data) => {
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static(constants::JSON_WITH_CHARSET),
                );
                (StatusCode::OK, headers, axum::Json(data)).into_response()
            }
            Response::Xml(xml) => {
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static(constants::XML_WITH_CHARSET),
                );
                (StatusCode::OK, headers, xml).into_response()
            }
            Response::PlainText(text) => {
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static(constants::TEXT_WITH_CHARSET),
                );
                (StatusCode::OK, headers, text).into_response()
            }
        }
    }
}
