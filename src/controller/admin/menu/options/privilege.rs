use crate::{
    common::error::Error,
    model::sys::menu::TABLE_SYS_MENU,
    utils::recursion::{Recursion, Recursive},
};
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

// 获取部门可选菜单
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PrivilegeOptions {
    #[serde(rename = "value")]
    pub id: i64,

    #[serde(skip)] // 不序列化
    pub pid: Option<i64>,

    // 标签
    #[serde(rename = "label")]
    pub title: Option<String>,

    // 类型
    pub enforce: bool,

    // 隐藏/可选
    #[serde(rename = "disabled")]
    pub hidden: bool,

    #[sqlx(skip)] // 不从数据库查询
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<PrivilegeOptions>>,
}

impl Recursive for PrivilegeOptions {
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

// 菜单和接口联级选项
pub async fn query_privilege_options(db: &MySqlPool) -> Result<Vec<PrivilegeOptions>, Error> {
    // 查询菜单
    let query = format!(
        r#"SELECT `id`, `pid`, `title`, `hidden`, `enforce` 
        FROM `{TABLE_SYS_MENU}` 
        WHERE `is_deleted` = 0 
        ORDER BY `order_no` ASC"#
    );
    let menus: Vec<PrivilegeOptions> = sqlx::query_as(&query).fetch_all(db).await?;

    Ok(Recursion::insert_childrens(menus)?)
}
