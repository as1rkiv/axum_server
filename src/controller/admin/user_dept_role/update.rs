use super::{delete, insert, UserPrivilege};
use crate::common::error::Error;
use sqlx::{
    mysql::{MySql, MySqlQueryResult},
    Transaction,
};

// 更新用户权限 - 事务
pub async fn update_user_dept_role_with_transaction(
    tx: &mut Transaction<'_, MySql>,
    user_id: i64,
    privileges: Vec<UserPrivilege>,
) -> Result<MySqlQueryResult, Error> {
    // 删除当前权限
    delete::delete_user_dept_role_with_transaction(tx, user_id).await?;

    // 插入新权限
    insert::insert_user_dept_role_with_transaction(tx, user_id, &privileges).await
}
