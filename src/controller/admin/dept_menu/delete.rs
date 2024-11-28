use crate::{common::error::Error, model::sys::dept_menu::TABLE_SYS_DEPT_MENU};
use sqlx::{
    mysql::{MySql, MySqlQueryResult},
    Acquire, QueryBuilder, Transaction,
};

// 删除部门菜单 - 事务
pub async fn delete_dept_menus_with_transaction(
    tx: &mut Transaction<'_, MySql>,
    dept_id: i64,
    menu_ids: &Vec<i64>,
) -> Result<MySqlQueryResult, Error> {
    let conn = tx.acquire().await?;

    // 构建删除信息
    let sql = format!(r#"DELETE FROM `{TABLE_SYS_DEPT_MENU}` WHERE `dept_id` = "#);
    let mut builder = QueryBuilder::new(sql);

    // 绑定部门ID
    builder.push_bind(dept_id);

    // 推入需要删除的菜单ID
    let mut separated = builder.separated(", ");
    separated.push_unseparated(" AND `menu_id` IN (");
    for menu_id in menu_ids.iter() {
        separated.push_bind(menu_id);
    }
    separated.push_unseparated(")"); // 关闭 IN 子句

    // 返回结果
    Ok(builder.build().execute(&mut *conn).await?)
}
