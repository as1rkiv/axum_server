use super::UserPrivilege;
use crate::{common::error::Error, model::sys::user_dept_role::TABLE_SYS_USER_DEPT_ROLE};
use sqlx::{
    mysql::{MySql, MySqlQueryResult},
    Acquire, QueryBuilder, Transaction,
};

// 添加用户权限 - 事务
pub async fn insert_user_dept_role_with_transaction(
    tx: &mut Transaction<'_, MySql>,
    user_id: i64,
    privileges: &Vec<UserPrivilege>,
) -> Result<MySqlQueryResult, Error> {
    // 获取事务链接
    let conn = tx.acquire().await?;

    // 构建插入信息
    let sql = format!(
        "INSERT INTO `{TABLE_SYS_USER_DEPT_ROLE}` (`user_id`, `dept_id`, `role_id`) VALUES "
    );
    let mut builder = QueryBuilder::new(sql);

    let mut separated = builder.separated(", ");
    for p in privileges.iter() {
        separated
            .push("(")
            .push_bind_unseparated(user_id)
            .push_bind(p.dept_id)
            .push_bind(p.role_id)
            .push_unseparated(")");
    }

    // 返回结果
    Ok(builder.build().execute(&mut *conn).await?)
}
