use crate::{
    common::error::Error, model::sys::role::TABLE_SYS_ROLE, store::mysql::QueryBuilderExt,
};
use serde::Serialize;
use sqlx::{query_builder::QueryBuilder, FromRow, MySqlPool};

// 角色选项
#[derive(Debug, Serialize, FromRow)]
pub struct RoleOptions {
    #[serde(rename = "value")]
    pub id: i64,

    #[serde(rename = "label")]
    pub name: String,
}

// 获取角色选项
pub async fn query_role_options(db: &MySqlPool) -> Result<Vec<RoleOptions>, Error> {
    let mut query = QueryBuilder::select("r.id, r.name", TABLE_SYS_ROLE, "r");

    // 构建查询
    let options: Vec<RoleOptions> = query
        .maybe()
        .and()
        .is_not_deleted("r")
        .build_query_as()
        .fetch_all(db)
        .await?;

    Ok(options)
}
