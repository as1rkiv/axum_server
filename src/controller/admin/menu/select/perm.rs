use super::super::PermListResp;
use crate::{
    common::{error::Error, request::Pagination},
    model::sys::menu::TABLE_SYS_MENU,
    store::mysql::QueryBuilderExt,
};
use serde::Deserialize;
use sqlx::{query_builder::QueryBuilder, MySqlPool};
use validator::Validate;

// 接口列表请求
#[derive(Debug, Default, Deserialize, Validate)]
pub struct PermListQuery {
    pid: Option<String>,
}

// 获取接口列表
pub async fn query_perm_list(
    db: &MySqlPool,
    params: Pagination<PermListQuery>,
) -> Result<(i64, Vec<PermListResp>), Error> {
    // 查询字段
    let mut query_builder = QueryBuilder::select(
        "p.id, p.pid, p.component, p.title, p.action, m.title AS menu",
        TABLE_SYS_MENU,
        "p",
    );

    // 链接 Menu 表获取菜单标题
    query_builder
        .join(format!("LEFT JOIN `{TABLE_SYS_MENU}` m ON m.id = p.pid"))
        .maybe()
        .and()
        .is_not_deleted("p")
        .and()
        .eq("p.enforce", true);

    // 查询数量
    let mut count_builder = QueryBuilder::select("COUNT(*) AS count", TABLE_SYS_MENU, "p");
    count_builder
        .maybe()
        .and()
        .is_not_deleted("p")
        .and()
        .eq("p.enforce", true);

    // 参数
    if let Some(menu_id) = &params.get_params().pid {
        if let Ok(id) = menu_id.parse::<i64>() {
            query_builder.and().eq("p.pid", id);
            count_builder.and().eq("p.pid", id);
        }
    }

    // 查询数量
    let count: i64 = match count_builder.build_query_scalar().fetch_one(db).await? {
        0 => return Ok((0, vec![])), // 数量为0时直接返回空列表
        count => count,
    };

    // 查询
    let list: Vec<PermListResp> = query_builder
        .order_by("m.order_no ASC, m.id ASC, p.id ASC")
        .pagination(params.get_index(), params.get_size())
        .build_query_as()
        .fetch_all(db)
        .await?;

    Ok((count, list))
}
