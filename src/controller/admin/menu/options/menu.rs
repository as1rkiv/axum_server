use crate::{
    common::error::Error,
    model::sys::menu::TABLE_SYS_MENU,
    store::mysql::QueryBuilderExt,
    utils::recursion::{Recursion, Recursive},
};
use serde::Serialize;
use sqlx::{query_builder::QueryBuilder, FromRow, MySqlPool};

// 菜单选项响应
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MenuOptions {
    #[serde(rename = "value")]
    pub id: i64,

    #[serde(skip)] // 不序列化
    pub pid: Option<i64>,

    #[serde(rename = "label")]
    pub title: Option<String>,

    #[sqlx(skip)] // 不从数据库查询
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<MenuOptions>>,
}

// 实现递归插入
impl Recursive for MenuOptions {
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

// 菜单联级选项
pub async fn query_menu_options(db: &MySqlPool) -> Result<Vec<MenuOptions>, Error> {
    // 构建查询
    let options: Vec<MenuOptions> =
        QueryBuilder::select("m.id, m.pid, m.title", TABLE_SYS_MENU, "m")
            .maybe()
            .and()
            .eq("m.enforce", false)
            .and()
            .is_not_deleted("m")
            .order_by("m.order_no ASC")
            .build_query_as()
            .fetch_all(db)
            .await?;

    Ok(Recursion::insert_childrens(options)?)
}
