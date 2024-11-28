use super::cache::get_user_privileges;
use crate::{
    common::error::Error,
    model::sys::{dept_menu::TABLE_SYS_DEPT_MENU, menu::TABLE_SYS_MENU},
    state::{user::UserState, AppState},
};
use std::collections::HashSet;

/// 检查访问权限
pub(super) async fn check(
    app_state: &AppState,
    user_state: &UserState,
    resource: &str,
    action: &str,
) -> Result<(), Error> {
    // 从缓存中获取用户拥有的所有部门角色和数据范围
    let drdsh = get_user_privileges(app_state, user_state.get_id()).await?;

    // 没有权限则失败
    if drdsh.is_empty() {
        return Err(Error::PermissionDenied);
    }

    // 部门ID 转逗号分隔的字符串，去除重复项
    let dept_ids = drdsh
        .iter()
        .map(|r| r.dept_id)
        .collect::<HashSet<i64>>() // 利用 HashSet 去重
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    // 构建sql
    let query = format!(
        r#"SELECT EXISTS ( SELECT 1 
        FROM `{TABLE_SYS_DEPT_MENU}` dm 
        INNER JOIN `{TABLE_SYS_MENU}` m 
        ON m.enforce = 1 AND m.id = dm.menu_id 
        WHERE m.component = ? AND m.action = ? 
        AND dm.dept_id IN ({dept_ids}) );"#
    );

    // 查询权限数量，传入角色 ID 列表，判断是否放行
    if sqlx::query_scalar(&query)
        .bind(resource)
        .bind(action)
        .fetch_one(app_state.mysql())
        .await?
    {
        return Ok(());
    }

    // 默认无权限
    Err(Error::PermissionDenied)
}
