pub mod delete;
pub mod insert;
pub mod select;
pub mod update;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

// 用户权限
#[derive(Debug, Clone, Deserialize, Serialize, Validate, FromRow)]
pub struct UserPrivilege {
    // 部门信息
    #[validate(required(message = "参数无效"), range(min = 1, message = "参数无效"))]
    dept_id: Option<i64>,
    dept_name: Option<String>,

    // 角色信息
    #[validate(required(message = "参数无效"), range(min = 1, message = "参数无效"))]
    role_id: Option<i64>,
    role_name: Option<String>,
}
