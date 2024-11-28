use crate::{
    common::error::Error,
    model::sys::dept::TABLE_SYS_DEPT,
    store::mysql::QueryBuilderExt,
    utils::recursion::{Recursion, Recursive},
};
use serde::Serialize;
use sqlx::{query_builder::QueryBuilder, FromRow, MySqlPool};

// 部门选项
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct DeptOptions {
    #[serde(rename = "value")]
    pub id: i64,

    #[serde(skip)]
    pub pid: Option<i64>,

    #[serde(rename = "label")]
    pub name: String,

    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DeptOptions>>,
}

// 实现递归插入
impl Recursive for DeptOptions {
    fn get_id(&self) -> i64 {
        self.id
    }
    fn get_pid(&self) -> Option<i64> {
        self.pid
    }
    fn get_children_mut(&mut self) -> &mut Option<Vec<Self>> {
        &mut self.children
    }
}

// 获取部门选项树
pub async fn query_dept_options(db: &MySqlPool) -> Result<Vec<DeptOptions>, Error> {
    // 构建查询
    let tree_list: Vec<DeptOptions> =
        QueryBuilder::select("d.id, d.pid, d.name", TABLE_SYS_DEPT, "d")
            .maybe()
            .and()
            .is_not_deleted("d")
            .build_query_as()
            .fetch_all(db)
            .await?;

    Ok(Recursion::insert_childrens(tree_list)?)
}
