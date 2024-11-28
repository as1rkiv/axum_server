use crate::{
    common::error::Error, middleware::privilege::cache::del_users_privileges,
    model::sys::dept::TABLE_SYS_DEPT, state::AppState,
};

// 软删除部门
pub async fn delete_dept_by_id(app_state: &AppState, dept_id: i64) -> Result<u64, Error> {
    let query = format!(r#"UPDATE `{TABLE_SYS_DEPT}` SET `is_deleted` = 1 WHERE `id` = ?"#);
    let result = sqlx::query(&query)
        .bind(dept_id)
        .execute(app_state.mysql())
        .await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    Ok(result.rows_affected())
}
