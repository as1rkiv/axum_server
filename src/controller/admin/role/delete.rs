use crate::{
    common::error::Error, middleware::privilege::cache::del_users_privileges,
    model::sys::role::TABLE_SYS_ROLE, state::AppState,
};

// 软删除角色
pub async fn delete_role_by_id(app_state: &AppState, role_id: i64) -> Result<u64, Error> {
    let query = format!(r#"UPDATE `{TABLE_SYS_ROLE}` SET `is_deleted` = 1 WHERE `id` = ?"#);
    let result = sqlx::query(&query)
        .bind(role_id)
        .execute(app_state.mysql())
        .await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    Ok(result.rows_affected())
}
