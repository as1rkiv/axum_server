use super::InsertOrUpdateDeptPayload;
use crate::{
    common::error::Error, controller::admin::dept_menu::insert::insert_dept_menus_with_transaction,
    middleware::privilege::cache::del_users_privileges, model::sys::dept::TABLE_SYS_DEPT,
    state::AppState, store::mysql::ISOLATION_SERIALIZABLE,
};
use sqlx::Acquire;

// 添加部门
pub async fn insert_dept(
    app_state: &AppState,
    created_by: i64,
    data: InsertOrUpdateDeptPayload,
) -> Result<u64, Error> {
    // 从连接池获取一个链接
    let mut conn = app_state.mysql().acquire().await?;

    // 设置最高隔离级别
    sqlx::query(ISOLATION_SERIALIZABLE)
        .execute(&mut *conn)
        .await?;

    // 开启事务
    let mut tx = conn.begin().await?;

    // 插入部门表
    let query = format!(
        r#"INSERT INTO `{TABLE_SYS_DEPT}` ( 
            `pid`, `name`, `desc`, `created_by` 
        ) VALUES (?, ?, ?, ?)"#
    );
    let result = sqlx::query(&query)
        .bind(data.pid)
        .bind(data.name.trim())
        .bind(data.desc.trim())
        .bind(created_by)
        .execute(&mut *tx)
        .await?;

    // 如果有
    if !data.perms.is_empty() {
        // 获取插入的ID
        let new_dept_id = result.last_insert_id() as i64;

        // 添加部门与菜单关联
        insert_dept_menus_with_transaction(&mut tx, new_dept_id, &data.perms).await?;
    }

    // 提交事务
    tx.commit().await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    // 返回影响行数
    Ok(result.rows_affected())
}
