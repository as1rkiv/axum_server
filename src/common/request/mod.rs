use crate::{
    common::error::Error,
    constants::{REQUEST_DEFAULT_INDEX, REQUEST_DEFAULT_PAGESIZE},
};
use axum::{
    async_trait,
    extract::{rejection::QueryRejection, FromRequestParts, Query},
    http::request::Parts,
};
use serde::{de::DeserializeOwned, Deserialize};
use validator::Validate;

// 通用分页请求
#[derive(Debug, Default, Deserialize)]
pub struct Pagination<T: Validate> {
    // 当前页
    #[serde(default, rename = "current")]
    page_index: Option<i64>,

    // 每页显示数量
    #[serde(default, rename = "pageSize")]
    page_size: Option<i64>,

    // 参数部分
    #[serde(flatten)]
    params: T,
}

impl<T: Validate> Pagination<T> {
    pub fn get_index(&self) -> i64 {
        self.page_index.unwrap_or(REQUEST_DEFAULT_INDEX)
    }
    pub fn get_size(&self) -> i64 {
        self.page_size.unwrap_or(REQUEST_DEFAULT_PAGESIZE)
    }
    pub fn get_params(&self) -> &T {
        &self.params
    }
}

// 分页查询提取器
#[async_trait]
impl<T, S> FromRequestParts<S> for Pagination<T>
where
    T: DeserializeOwned + Validate + Send + Sync,
    S: Send + Sync,
    Query<Pagination<T>>: FromRequestParts<S, Rejection = QueryRejection>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<Pagination<T>>::from_request_parts(parts, state).await?;
        value.params.validate()?;
        Ok(value)
    }
}

// 错误转换
impl From<QueryRejection> for Error {
    fn from(value: QueryRejection) -> Self {
        Self::Rejection(value.body_text())
    }
}
