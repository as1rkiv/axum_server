use super::InsertOrUpdateDeptPayload;
use crate::{
    common::error::Error, controller::admin::dept_menu::update::update_dept_menus,
    middleware::privilege::cache::del_users_privileges, model::sys::dept::TABLE_SYS_DEPT,
    state::AppState, store::mysql::ISOLATION_SERIALIZABLE, utils::recursion::Recursion,
};
use sqlx::Acquire;

// 更新部门 - 事务
pub async fn update_dept_by_id(
    app_state: &AppState,
    dept_id: i64,
    data: InsertOrUpdateDeptPayload,
) -> Result<u64, Error> {
    // 从连接池中手动获取一个连接
    // 确保隔离级别设置在当前连接即将开始的事务中
    // 而不是连接池中的其他连接
    let mut conn = app_state.mysql().acquire().await?;

    // 设置最高隔离级别
    sqlx::query(ISOLATION_SERIALIZABLE)
        .execute(&mut *conn)
        .await?;

    // 开启事务
    let mut tx = conn.begin().await?;

    // 如果有 pid，检查循环依赖
    if let Some(new_pid) = data.pid {
        if Recursion::has_circular_dependency(&mut tx, TABLE_SYS_DEPT, dept_id, new_pid).await? {
            tx.rollback().await?; // 回滚事务
            return Err(Error::Tips("检测到循环依赖".into()));
        }
    }

    // 更新部门
    let query = format!(
        r#"UPDATE `{TABLE_SYS_DEPT}` SET `pid` = ?, `name` = ?, `desc` = ? WHERE `id` = ?"#
    );
    let result = sqlx::query(&query)
        .bind(data.pid)
        .bind(data.name.trim())
        .bind(data.desc.trim())
        .bind(dept_id)
        .execute(&mut *tx)
        .await?;

    // 更新部门菜单
    update_dept_menus(&mut tx, dept_id, data.perms).await?;

    // 提交事务
    tx.commit().await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    // 返回影响行数
    Ok(result.rows_affected())
}
