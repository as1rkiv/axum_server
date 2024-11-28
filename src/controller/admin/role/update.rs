use super::InsertOrUpdateRolePayload;
use crate::{
    common::error::Error, middleware::privilege::cache::del_users_privileges,
    model::sys::role::TABLE_SYS_ROLE, state::AppState,
};

// 更新角色
pub async fn update_role_by_id(
    app_state: &AppState,
    role_id: i64,
    data: InsertOrUpdateRolePayload,
) -> Result<u64, Error> {
    // 更新部门
    let query = format!(
        r#"UPDATE `{TABLE_SYS_ROLE}` SET 
        `name` = ?, `data_scope` = ?, `desc` = ? 
        WHERE `id` = ?"#
    );
    let result = sqlx::query(&query)
        .bind(data.name.trim())
        .bind(data.data_scope)
        .bind(data.desc.trim())
        .bind(role_id)
        .execute(app_state.mysql())
        .await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    // 返回影响行数
    Ok(result.rows_affected())
}
