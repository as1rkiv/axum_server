#![allow(dead_code)]

use crate::utils::recursion::Recursive;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

/// 菜单表名
pub const TABLE_SYS_MENU: &'static str = "sys_menu";

// 权限
#[derive(Debug, Clone, FromRow)]
pub struct Perm {
    pub id: i64,

    // 菜单ID
    pub pid: i64,

    // 资源
    pub component: String,

    // 动作
    pub action: String,

    // 描述
    pub title: String,
}

// 菜单
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Menu {
    pub id: i64,

    // 父级
    pub pid: Option<i64>,

    // 路径
    pub path: String,

    // 名称
    pub name: String,

    // 组件
    pub component: String,

    // 重定向
    pub redirect: Option<String>,

    // 名称
    pub title: String,

    // 图标
    pub icon: Option<String>,

    // 顺序
    #[serde(rename = "orderNo")]
    pub order_no: Option<i16>,

    // 是否单独显示
    pub single: Option<bool>,

    // 是否展开
    pub expanded: Option<bool>,

    // 是否隐藏
    pub hidden: Option<bool>,

    // 是否隐藏面包屑
    #[serde(rename = "hiddenBreadcrumb")]
    pub hidden_breadcrumb: Option<bool>,

    // 是否保持活跃状态
    #[serde(rename = "keepAlive")]
    pub keep_alive: Option<bool>,

    // 外部链接地址
    #[serde(rename = "frameSrc")]
    pub frame_src: Option<String>,

    // 外部链接是否新窗口打开
    #[serde(rename = "frameBlank")]
    pub frame_blank: Option<bool>,

    // 创建人
    #[sqlx(skip)]
    #[serde(skip)]
    pub created_by: i64,

    // 创建时间
    #[sqlx(skip)]
    #[serde(skip)]
    pub created_at: DateTime<Utc>,

    // 更新时间
    #[sqlx(skip)]
    #[serde(skip)]
    pub updated_at: DateTime<Utc>,

    // 子菜单 - 跳过 sqlx 映射
    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Self>>,
}

// 菜单递归插入
impl Recursive for Menu {
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
