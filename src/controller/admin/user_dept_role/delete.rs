use crate::{common::error::Error, model::sys::user_dept_role::TABLE_SYS_USER_DEPT_ROLE};
use sqlx::{
    mysql::{MySql, MySqlQueryResult},
    Acquire, Transaction,
};

// 删除用户权限 - 事务
pub async fn delete_user_dept_role_with_transaction(
    tx: &mut Transaction<'_, MySql>,
    user_id: i64,
) -> Result<MySqlQueryResult, Error> {
    let conn = tx.acquire().await?;

    // 构建删除信息
    let query = format!(r#"DELETE FROM `{TABLE_SYS_USER_DEPT_ROLE}` WHERE `user_id` = ?"#);

    // 返回结果
    Ok(sqlx::query(&query)
        .bind(user_id)
        .execute(&mut *conn)
        .await?)
}
