pub mod delete;
pub mod insert;
pub mod options;
pub mod select;
pub mod update;

use serde::Deserialize;
use validator::Validate;

// 添加或更新部门
#[derive(Debug, Deserialize, Validate)]
pub struct InsertOrUpdateDeptPayload {
    // 父部门ID
    #[validate(range(min = 1, message = "参数无效"))]
    pub pid: Option<i64>,

    // 名称
    #[validate(length(min = 1, message = "参数无效"))]
    pub name: String,

    // 描述
    pub desc: String,

    // 菜单id
    pub perms: Vec<i64>,
}
