pub mod delete;
pub mod insert;
pub mod select;
pub mod update;

use crate::{controller::admin::user_dept_role::UserPrivilege, model::sys::user::User};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

// 添加或更新用户
#[derive(Debug, Deserialize, Validate)]
pub struct InsertOrUpdateUserPayload {
    // 姓名
    #[validate(length(min = 1, message = "参数无效"))]
    pub fullname: String,
    // 手机号
    #[validate(length(min = 1, message = "参数无效"))]
    pub username: String,
    // 密码
    #[validate(length(min = 1, message = "参数无效"))]
    pub password: String,
    // 账号状态
    pub is_active: bool,
    // 权限
    #[validate(nested)]
    pub privileges: Vec<UserPrivilege>,
}

// 用户列表响应
#[derive(Debug, Serialize, FromRow)]
pub struct UserListResp {
    // 用户信息
    #[serde(flatten)]
    pub user: User,
    // 权限
    #[sqlx(skip)]
    pub privileges: Vec<UserPrivilege>,
}
