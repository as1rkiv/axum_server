use super::{super::JsonData, super::Response};
use serde::Serialize;

// 通用列表响应 - 不带数量
#[derive(Debug, Clone, Default, Serialize)]
pub struct SingleList<T: Serialize> {
    pub list: Vec<T>,
}

// 转换到json
impl<T: Serialize> From<SingleList<T>> for JsonData<SingleList<T>> {
    fn from(value: SingleList<T>) -> Self {
        JsonData::value(value)
    }
}

// Vec 转换响应
impl<T: Serialize> From<Vec<T>> for Response<SingleList<T>> {
    fn from(value: Vec<T>) -> Self {
        Response::Json(SingleList { list: value }.into())
    }
}
