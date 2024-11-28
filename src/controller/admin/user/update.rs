use super::{super::user_dept_role::update, InsertOrUpdateUserPayload};
use crate::{
    common::error::Error, middleware::privilege::cache::del_user_privileges_by_id,
    model::sys::user::TABLE_SYS_USER, state::AppState,
};

// 更新用户
pub async fn update_user_by_id(
    app_state: &AppState,
    user_id: i64,
    data: InsertOrUpdateUserPayload,
) -> Result<u64, Error> {
    // 开启事务
    let mut tx = app_state.mysql().begin().await?;

    // 更新部门
    let query = format!(
        r#"UPDATE `{TABLE_SYS_USER}` SET 
        `username` = ?, `password` = ?, `fullname` = ? , `is_active` = ? 
        WHERE `id` = ?"#
    );
    let result = sqlx::query(&query)
        .bind(data.username.trim())
        .bind(data.password.trim())
        .bind(data.fullname.trim())
        .bind(data.is_active)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

    // 更新用户权限
    update::update_user_dept_role_with_transaction(&mut tx, user_id, data.privileges).await?;

    // 提交事务
    tx.commit().await?;

    // 删除缓存
    del_user_privileges_by_id(app_state, user_id).await?;

    // 返回影响行数
    Ok(result.rows_affected())
}
