pub mod delete;
pub mod insert;
pub mod options;
pub mod select;
pub mod update;

use serde::Deserialize;
use validator::Validate;

// 添加或更新角色
#[derive(Debug, Deserialize, Validate)]
pub struct InsertOrUpdateRolePayload {
    // 角色名
    #[validate(length(min = 1, message = "参数无效"))]
    pub name: String,

    // 数据访问范围
    #[validate(range(min = 0, max = 2, message = "参数无效"))]
    pub data_scope: u8,

    // 角色描述
    pub desc: String,
}
