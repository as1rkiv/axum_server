use crate::utils::recursion::Recursive;
use serde::Serialize;
use sqlx::{mysql::MySqlRow, Row};

// 菜单路由
#[derive(Debug, Clone, Serialize)]
pub struct MenuRoute {
    pub id: i64,
    #[serde(skip_serializing)] // 跳过 pid 字段的序列化
    pub pid: Option<i64>,
    // 路径
    pub path: String,
    // 名称
    pub name: String,
    // 组件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    // 重定向
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<String>,
    // 元数据
    pub meta: MenuMeta,
    // 子菜单
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Self>>,
}

// 菜单元数据
#[derive(Debug, Clone, Serialize)]
pub struct MenuMeta {
    // 名称
    pub title: String,
    // 图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    // 顺序
    #[serde(rename = "orderNo", skip_serializing_if = "Option::is_none")]
    pub order_no: Option<i16>,
    // 是否单独显示
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single: Option<bool>,
    // 是否展开
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    // 是否隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    // 是否隐藏面包屑
    #[serde(rename = "hiddenBreadcrumb", skip_serializing_if = "Option::is_none")]
    pub hidden_breadcrumb: Option<bool>,
    // 是否保持活跃状态
    #[serde(rename = "keepAlive", skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<bool>,
    // 外部链接地址
    #[serde(rename = "frameSrc", skip_serializing_if = "Option::is_none")]
    pub frame_src: Option<String>,
    // 外部链接是否新窗口打开
    #[serde(rename = "frameBlank", skip_serializing_if = "Option::is_none")]
    pub frame_blank: Option<bool>,
}

// 菜单路由递归插入
impl Recursive for MenuRoute {
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

// 实现 From<MySqlRow> trait
impl From<MySqlRow> for MenuRoute {
    fn from(row: MySqlRow) -> Self {
        Self {
            id: row.get("id"),
            pid: row.get("pid"),
            path: row.get("path"),
            name: row.get("name"),
            component: row.get("component"),
            redirect: row.get("redirect"),
            meta: MenuMeta {
                title: row.get("title"),
                icon: row.get("icon"),
                order_no: row.get("order_no"),
                single: row.get("single"),
                expanded: row.get("expanded"),
                hidden: row.get("hidden"),
                hidden_breadcrumb: row.get("hidden_breadcrumb"),
                keep_alive: row.get("keep_alive"),
                frame_src: row.get("frame_src"),
                frame_blank: row.get("frame_blank"),
            },
            children: None,
        }
    }
}
