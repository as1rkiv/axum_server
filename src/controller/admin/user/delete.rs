use super::super::user_dept_role::delete;
use crate::{
    common::error::Error, middleware::privilege::cache::del_user_privileges_by_id,
    model::sys::user::TABLE_SYS_USER, state::AppState,
};

// 软删除用户
pub async fn delete_user_by_id(app_state: &AppState, user_id: i64) -> Result<u64, Error> {
    // 开启事务
    let mut tx = app_state.mysql().begin().await?;

    // 软删除用户信息
    let query = format!(r#"UPDATE `{TABLE_SYS_USER}` SET `is_deleted` = 1 WHERE `id` = ?"#);
    let result = sqlx::query(&query).bind(user_id).execute(&mut *tx).await?;

    // 真实删除用户部门角色关联表
    delete::delete_user_dept_role_with_transaction(&mut tx, user_id).await?;

    // 提交事务
    tx.commit().await?;

    // 删除缓存
    del_user_privileges_by_id(app_state, user_id).await?;

    // 返回受影响行数
    Ok(result.rows_affected())
}
