#![allow(dead_code)]

use sqlx::FromRow;

/// 用户部门角色表名
pub const TABLE_SYS_USER_DEPT_ROLE: &str = "sys_user_dept_role";

#[derive(Debug, Clone, Copy, FromRow)]
pub struct UserDeptRole {
    // 用户ID
    pub user_id: i64,
    // 部门ID
    pub dept_id: i64,
    // 角色ID
    pub role_id: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserDeptRoleWithName {
    // 用户ID
    pub user_id: i64,

    // 部门ID
    pub dept_id: i64,
    pub dept_name: Option<String>,

    // 角色ID
    pub role_id: i64,
    pub role_name: Option<String>,
}
