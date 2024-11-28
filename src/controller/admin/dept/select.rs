use crate::{
    common::error::Error,
    model::sys::dept::{Dept, TABLE_SYS_DEPT},
    store::mysql::QueryBuilderExt,
    utils::recursion::Recursion,
};
use serde::Deserialize;
use sqlx::{query_builder::QueryBuilder, MySqlPool};
use validator::Validate;

// 菜单请求
#[derive(Debug, Deserialize, Validate)]
pub struct DeptTreeQuery {
    name: Option<String>,
}

// 获取部门树
pub async fn query_dept_tree(db: &MySqlPool, params: DeptTreeQuery) -> Result<Vec<Dept>, Error> {
    // 查询字段
    let mut query_builder = QueryBuilder::select("d.*", TABLE_SYS_DEPT, "d");

    query_builder.maybe().and().is_not_deleted("d");

    // 参数
    if let Some(name) = &params.name {
        if !name.is_empty() {
            query_builder.and().like("d.name", &name.trim());
        }
    }

    // 构建查询
    let tree_list: Vec<Dept> = query_builder.build_query_as().fetch_all(db).await?;

    Ok(Recursion::insert_childrens(tree_list)?)
}
