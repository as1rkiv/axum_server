#![allow(dead_code)]

use sqlx::FromRow;

/// 部门菜单表名
pub const TABLE_SYS_DEPT_MENU: &str = "sys_dept_menu";

#[derive(Debug, Clone, Copy, FromRow)]
pub struct DeptMenu {
    // 部门ID
    pub dept_id: i64,
    // 菜单ID
    pub menu_id: i64,
}
