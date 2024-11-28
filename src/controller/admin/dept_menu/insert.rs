use crate::{common::error::Error, model::sys::dept_menu::TABLE_SYS_DEPT_MENU};
use sqlx::{
    mysql::{MySql, MySqlQueryResult},
    Acquire, QueryBuilder, Transaction,
};

// 添加部门菜单表 - 事务
pub async fn insert_dept_menus_with_transaction(
    tx: &mut Transaction<'_, MySql>,
    dept_id: i64,
    menu_ids: &Vec<i64>,
) -> Result<MySqlQueryResult, Error> {
    // 获取事务链接
    let conn = tx.acquire().await?;

    // 构建插入信息
    let sql = format!("INSERT INTO `{TABLE_SYS_DEPT_MENU}` (`dept_id`, `menu_id`) VALUES ");
    let mut builder = QueryBuilder::new(sql);

    let mut separated = builder.separated(", ");
    for menu_id in menu_ids.iter() {
        separated
            .push("(")
            .push_bind_unseparated(dept_id)
            .push_bind(menu_id)
            .push_unseparated(")");
    }

    // 返回结果
    Ok(builder.build().execute(&mut *conn).await?)
}
