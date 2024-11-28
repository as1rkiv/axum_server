use super::{super::user_dept_role::insert, InsertOrUpdateUserPayload};
use crate::{common::error::Error, model::sys::user::TABLE_SYS_USER, state::AppState};

// 添加用户
pub async fn insert_user(
    app_state: &AppState,
    created_by: i64,
    data: InsertOrUpdateUserPayload,
) -> Result<u64, Error> {
    // 开启事务
    let mut tx = app_state.mysql().begin().await?;

    // 插入用户表
    let query = format!(
        r#"INSERT INTO `{TABLE_SYS_USER}` ( 
            `username`, `password`, `fullname`, `is_active`, `created_by`
        ) VALUES (?, ?, ?, ?, ?)"#
    );
    let result = sqlx::query(&query)
        .bind(data.username.trim())
        .bind(data.password.trim())
        .bind(data.fullname.trim())
        .bind(data.is_active)
        .bind(created_by)
        .execute(&mut *tx)
        .await?;

    // 如果有
    if !data.privileges.is_empty() {
        // 获取插入的user_id
        let user_id = result.last_insert_id() as i64;

        // 添加用户部门角色关联表
        insert::insert_user_dept_role_with_transaction(&mut tx, user_id, &data.privileges).await?;
    }

    // 提交事务
    tx.commit().await?;

    // 返回影响行数
    Ok(result.rows_affected())
}
