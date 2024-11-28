use super::super::InsertOrUpdatePermPayload;
use crate::{
    common::error::Error, middleware::privilege::cache::del_users_privileges,
    model::sys::menu::TABLE_SYS_MENU, state::AppState,
};

// 添加接口
pub async fn insert_perm(
    app_state: &AppState,
    created_by: i64,
    data: InsertOrUpdatePermPayload,
) -> Result<u64, Error> {
    let query = format!(
        r#"INSERT INTO `{TABLE_SYS_MENU}` ( 
            `pid`, `component`, `action`, `title`, `enforce`, `created_by` 
        ) VALUES (?, ?, ?, ?, ?, ?)"#
    );
    let result = sqlx::query(&query)
        .bind(data.pid)
        .bind(data.component.trim())
        .bind(data.action.trim())
        .bind(data.title.trim())
        .bind(true)
        .bind(created_by)
        .execute(app_state.mysql())
        .await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    Ok(result.rows_affected())
}
