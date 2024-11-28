use super::InsertOrUpdateRolePayload;
use crate::{
    common::error::Error, middleware::privilege::cache::del_users_privileges,
    model::sys::role::TABLE_SYS_ROLE, state::AppState,
};

// 添加角色
pub async fn insert_role(
    app_state: &AppState,
    created_by: i64,
    data: InsertOrUpdateRolePayload,
) -> Result<u64, Error> {
    let query = format!(
        r#"INSERT INTO `{TABLE_SYS_ROLE}` ( 
            `name`, `data_scope`, `desc`,  `created_by`
        ) VALUES (?, ?, ?, ?)"#
    );
    let result = sqlx::query(&query)
        .bind(data.name.trim())
        .bind(data.data_scope)
        .bind(data.desc.trim())
        .bind(created_by)
        .execute(app_state.mysql())
        .await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    Ok(result.rows_affected())
}
