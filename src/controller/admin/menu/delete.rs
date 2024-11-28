use crate::{
    common::error::Error, middleware::privilege::cache::del_users_privileges,
    model::sys::menu::TABLE_SYS_MENU, state::AppState,
};

// 软删除菜单
pub async fn delete_menu_or_perm_by_id(app_state: &AppState, menu_id: i64) -> Result<u64, Error> {
    let query = format!(r#"UPDATE `{TABLE_SYS_MENU}` SET `is_deleted` = 1 WHERE `id` = ?"#);
    let result = sqlx::query(&query)
        .bind(menu_id)
        .execute(app_state.mysql())
        .await?;

    // 删除缓存
    del_users_privileges(app_state).await?;

    Ok(result.rows_affected())
}
