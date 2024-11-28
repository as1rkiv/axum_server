pub mod delete;
pub mod insert;
pub mod options;
pub mod select;
pub mod update;

use serde::Deserialize;
use validator::Validate;

// redis缓存key
const REGION_CACHE_KEY: &str = "REGION";

// 添加或更新区域
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct InsertOrUpdateOptRegion {
    pid: Option<i64>,

    #[validate(length(min = 1, message = "参数无效"))]
    name: String,

    #[validate(length(min = 6, max = 9, message = "参数无效"))]
    code: String,
}
