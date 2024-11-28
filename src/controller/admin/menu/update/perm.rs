use super::super::InsertOrUpdatePermPayload;
use crate::{
    common::error::Error, middleware::privilege::cache::del_users_privileges,
    model::sys::menu::TABLE_SYS_MENU, state::AppState,
};

// 更新接口
pub async fn update_perm_by_id(
    app_state: &AppState,
    perm_id: i64,
    data: InsertOrUpdatePermPayload,
) -> Result<u64, Error> {
    // 更新部门
    let query = format!(
        r#"UPDATE `{TABLE_SYS_MENU}` SET 
        `pid` = ?, `component` = ?, `action` = ?, `title` = ? 
        WHERE `id` = ?"#
    );
    let result = sqlx::query(&query)
        .bind(data.pid)
        .bind(data.component.trim())
        .bind(data.action.trim())
        .bind(data.title.trim())
        .bind(perm_id)
        .execute(app_state.mysql())
        .await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    // 返回影响行数
    Ok(result.rows_affected())
}
