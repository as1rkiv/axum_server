use crate::{
    common::error::Error, model::sys::dept_menu::TABLE_SYS_DEPT_MENU, store::mysql::QueryBuilderExt,
};
use sqlx::{query_builder::QueryBuilder, Acquire, MySql, MySqlPool, Transaction};

// 获取部门当前菜单
pub async fn query_dept_menus(db: &MySqlPool, dept_id: i64) -> Result<Vec<i64>, Error> {
    if dept_id <= 0 {
        return Ok(vec![]);
    }

    // 查询字段
    let result = QueryBuilder::select("dm.menu_id", TABLE_SYS_DEPT_MENU, "dm")
        .maybe()
        .and()
        .eq("dm.dept_id", dept_id)
        .build_query_scalar()
        .fetch_all(db)
        .await?;

    // 构建查询
    Ok(result)
}

// 获取部门当前菜单 - 事务
pub async fn query_dept_menus_with_transaction(
    tx: &mut Transaction<'_, MySql>,
    dept_id: i64,
) -> Result<Vec<i64>, Error> {
    if dept_id <= 0 {
        return Ok(vec![]);
    }

    // 获取事务连接
    let conn = tx.acquire().await?;

    // 查询字段
    let res = QueryBuilder::select("dm.menu_id", TABLE_SYS_DEPT_MENU, "dm")
        .maybe()
        .and()
        .eq("dm.dept_id", dept_id)
        .build_query_scalar()
        .fetch_all(&mut *conn)
        .await?;

    Ok(res)
}
