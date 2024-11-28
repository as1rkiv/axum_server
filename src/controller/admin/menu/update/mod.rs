mod perm;

use super::InsertOrUpdateMenuPayload;
use crate::{
    common::error::Error, middleware::privilege::cache::del_users_privileges,
    model::sys::menu::TABLE_SYS_MENU, state::AppState, store::mysql::ISOLATION_SERIALIZABLE,
    utils::recursion::Recursion,
};
use sqlx::Acquire;

// 导出更新接口
pub use perm::update_perm_by_id;

// 更新菜单 - 事务
pub async fn update_menu_by_id(
    app_state: &AppState,
    menu_id: i64,
    data: InsertOrUpdateMenuPayload,
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
        if Recursion::has_circular_dependency(&mut tx, TABLE_SYS_MENU, menu_id, new_pid).await? {
            tx.rollback().await?; // 回滚事务
            return Err(Error::Tips("检测到循环依赖".into()));
        }
    }

    // 更新菜单
    let query = format!(
        r#"UPDATE `{TABLE_SYS_MENU}` SET 
        `pid` = ?, `path` = ?, `name` = ?, `component` = ?, 
        `redirect` = ?, `title` = ?, `icon` = ?, `order_no` = ?, 
        `single` = ?, `expanded` = ?, `hidden` = ?, `hidden_breadcrumb` = ?, 
        `keep_alive` = ?, `frame_src` = ?, `frame_blank` = ? 
        WHERE `id` = ?"#
    );
    let result = sqlx::query(&query)
        .bind(data.pid)
        .bind(data.path.trim())
        .bind(data.name.trim())
        .bind(data.component.filter(|s| !s.is_empty()))
        .bind(data.redirect.filter(|s| !s.is_empty()))
        .bind(data.title.trim())
        .bind(data.icon.filter(|s| !s.is_empty()))
        .bind(data.order_no)
        .bind(data.single)
        .bind(data.expanded)
        .bind(data.hidden)
        .bind(data.hidden_breadcrumb)
        .bind(data.keep_alive)
        .bind(data.frame_src)
        .bind(data.frame_blank)
        .bind(menu_id)
        .execute(&mut *tx)
        .await?;

    // 提交事务
    tx.commit().await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    // 返回影响行数
    Ok(result.rows_affected())
}
