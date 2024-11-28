use super::{delete, insert, select};
use crate::common::error::Error;
use sqlx::{mysql::MySql, Transaction};
use std::collections::HashSet;

// 更新部门菜单 - 事务
pub async fn update_dept_menus(
    tx: &mut Transaction<'_, MySql>,
    dept_id: i64,
    menu_ids: Vec<i64>,
) -> Result<(), Error> {
    // 获取当前数据库中的部门菜单ID
    let exists_menus: HashSet<i64> = select::query_dept_menus_with_transaction(tx, dept_id)
        .await?
        .into_iter()
        .collect();

    // 获取需要修改的部门菜单ID
    let new_menus: HashSet<i64> = menu_ids.iter().copied().collect();

    // 需要插入的菜单ID
    let to_insert: Vec<i64> = new_menus.difference(&exists_menus).copied().collect();

    // 需要删除的菜单ID
    let to_delete: Vec<i64> = exists_menus.difference(&new_menus).copied().collect();

    // 插入菜单
    if !to_insert.is_empty() {
        insert::insert_dept_menus_with_transaction(tx, dept_id, &to_insert).await?;
    }

    // 删除菜单
    if !to_delete.is_empty() {
        delete::delete_dept_menus_with_transaction(tx, dept_id, &to_delete).await?;
    }

    Ok(())
}
