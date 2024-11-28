mod structure;

use crate::{
    common::error::Error,
    middleware::privilege::cache::get_user_privileges,
    model::sys::{dept_menu::TABLE_SYS_DEPT_MENU, menu::TABLE_SYS_MENU},
    state::{user::UserState, AppState},
    utils::recursion::Recursion,
};

// 导出结构体
pub use structure::MenuRoute;

// 根据部门返回菜单
pub async fn fetch_menus_by_user(
    app_state: &AppState,
    user_state: &UserState,
) -> Result<Vec<MenuRoute>, Error> {
    // 根据权限判断菜单
    let menus = if user_state.is_admin() {
        let query = format!(
            r#"SELECT * FROM `{TABLE_SYS_MENU}` 
            WHERE `enforce` = 0 AND `is_deleted` = 0 
            ORDER BY `order_no` ASC"#
        );
        sqlx::query(&query)
            .fetch_all(app_state.mysql())
            .await?
            .into_iter()
            .map(MenuRoute::from)
            .collect()
    } else {
        // 从缓存中获取继承的所有部门角色和数据范围
        let dept_ids = get_user_privileges(app_state, user_state.get_id())
            .await?
            .iter()
            .map(|r| r.dept_id.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        println!("!!!!!!!: {}", dept_ids);

        // 如果 role_ids 为空，直接返回空 Vec
        if dept_ids.is_empty() {
            return Ok(vec![]);
        }

        // 构建查询
        let query = format!(
            r#"SELECT DISTINCT m.* FROM `{TABLE_SYS_DEPT_MENU}` rm 
            LEFT JOIN `{TABLE_SYS_MENU}` m ON m.id = rm.menu_id 
            WHERE m.is_deleted = 0 AND m.enforce = 0 
            AND rm.dept_id IN ({dept_ids}) 
            ORDER BY m.order_no ASC"#
        );

        sqlx::query(&query)
            .fetch_all(app_state.mysql())
            .await?
            .into_iter()
            .map(MenuRoute::from)
            .collect()
    };

    // 递归插入父菜单
    Ok(Recursion::insert_childrens(menus)?)
}
