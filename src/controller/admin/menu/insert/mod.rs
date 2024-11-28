mod perm;

use super::InsertOrUpdateMenuPayload;
use crate::{
    common::error::Error, middleware::privilege::cache::del_users_privileges,
    model::sys::menu::TABLE_SYS_MENU, state::AppState,
};

// 导出添加接口
pub use perm::insert_perm;

// 添加菜单
pub async fn insert_menu(
    app_state: &AppState,
    created_by: i64,
    data: InsertOrUpdateMenuPayload,
) -> Result<u64, Error> {
    let query = format!(
        r#"INSERT INTO `{TABLE_SYS_MENU}` ( 
            `pid`, `path`, `name`, `component`, `redirect`, `title` , `icon`, 
            `order_no`, `single`, `expanded`, `hidden`, `hidden_breadcrumb`, 
            `keep_alive`, `frame_src`, `frame_blank`, `action`, `created_by` 
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
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
        .bind("ROUTE")
        .bind(created_by)
        .execute(app_state.mysql())
        .await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    Ok(result.rows_affected())
}
