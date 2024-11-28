use super::{super::Response, JsonData};
use serde::Serialize;

// 通用通知响应
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct Notify {
    pub affected: u64,
}

// 通知转json
impl From<Notify> for JsonData<Notify> {
    fn from(value: Notify) -> Self {
        Self::value(value)
    }
}

// 直接从u64转到响应
impl From<u64> for Response<Notify> {
    fn from(value: u64) -> Self {
        Self::Json(Notify { affected: value }.into())
    }
}
