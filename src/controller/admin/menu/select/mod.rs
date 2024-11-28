mod perm;

use crate::{
    common::error::Error,
    model::sys::menu::{Menu, TABLE_SYS_MENU},
    store::mysql::QueryBuilderExt,
    utils::recursion::Recursion,
};
use serde::Deserialize;
use sqlx::{query_builder::QueryBuilder, MySqlPool};
use validator::Validate;

// 导出接口查询
pub use perm::{query_perm_list, PermListQuery};

// 菜单请求
#[derive(Debug, Deserialize, Validate)]
pub struct MenuTreeListQuery {
    title: Option<String>,
}

// 获取菜单列表
pub async fn query_menu_tree_list(
    db: &MySqlPool,
    params: MenuTreeListQuery,
) -> Result<Vec<Menu>, Error> {
    // 查询字段
    let mut query_builder = QueryBuilder::select("m.*", TABLE_SYS_MENU, "m");

    query_builder.maybe().and().eq("m.enforce", false);

    // 参数
    if let Some(title) = &params.title {
        if !title.is_empty() {
            query_builder.and().like("m.title", title.trim());
        }
    }

    // 构建查询
    let menus: Vec<Menu> = query_builder
        .and()
        .is_not_deleted("m")
        .order_by("m.order_no ASC")
        .build_query_as()
        .fetch_all(db)
        .await?;

    Ok(Recursion::insert_childrens(menus)?)
}
