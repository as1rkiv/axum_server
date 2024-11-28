use crate::{
    common::{error::Error, request::Pagination},
    model::sys::role::{Role, TABLE_SYS_ROLE},
    store::mysql::QueryBuilderExt,
};
use serde::Deserialize;
use sqlx::{MySqlPool, QueryBuilder};
use validator::Validate;

// 角色卡片列表查询项
#[derive(Debug, Deserialize, Validate)]
pub struct RoleCardQuery {
    pub name: Option<String>,
}

// 获取角色卡片以及直接继承关系
pub async fn query_role_card(
    db: &MySqlPool,
    params: Pagination<RoleCardQuery>,
) -> Result<(i64, Vec<Role>), Error> {
    // 查询数量
    let mut count_builder = QueryBuilder::select("COUNT(r.id) AS count", TABLE_SYS_ROLE, "r");

    // 查询字段
    let mut query_builder = QueryBuilder::select("r.*", TABLE_SYS_ROLE, "r");

    count_builder.maybe().and().is_not_deleted("r");
    query_builder.maybe().and().is_not_deleted("r");

    // 角色名
    if let Some(name) = &params.get_params().name {
        if !name.is_empty() {
            count_builder.and().like("r.name", name.trim());
            query_builder.and().like("r.name", name.trim());
        }
    }

    // 查询数量
    let count: i64 = match count_builder.build_query_scalar().fetch_one(db).await? {
        0 => return Ok((0, vec![])), // 数量为0时直接返回空列表
        count => count,
    };

    // 获取所有角色
    let roles: Vec<Role> = query_builder
        .order_by("r.id DESC")
        .pagination(params.get_index(), params.get_size())
        .build_query_as()
        .fetch_all(db)
        .await?;

    Ok((count, roles))
}
