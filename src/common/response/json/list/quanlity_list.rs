use super::{super::JsonData, super::Response};
use serde::Serialize;

// 通用列表响应 - 带数量
#[derive(Debug, Clone, Default, Serialize)]
pub struct QuantityList<T: Serialize> {
    pub total: i64,
    pub list: Vec<T>,
}

// 转换到json
impl<T: Serialize> From<QuantityList<T>> for JsonData<QuantityList<T>> {
    fn from(value: QuantityList<T>) -> Self {
        JsonData::value(value)
    }
}

// 元组 Vec 转换响应
impl<T: Serialize> From<(i64, Vec<T>)> for Response<QuantityList<T>> {
    fn from(value: (i64, Vec<T>)) -> Self {
        Response::Json(
            QuantityList {
                total: value.0,
                list: value.1,
            }
            .into(),
        )
    }
}
