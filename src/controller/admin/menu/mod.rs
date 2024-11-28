pub mod delete;
pub mod insert;
pub mod options;
pub mod select;
pub mod update;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

// 添加或更新接口权限
#[derive(Debug, Deserialize, Validate)]
pub struct InsertOrUpdatePermPayload {
    // 菜单ID
    #[validate(range(min = 1, message = "参数无效"))]
    pub pid: i64,
    // 资源
    #[validate(length(min = 1, message = "参数无效"))]
    pub component: String,
    // 动作
    #[validate(length(min = 1, message = "参数无效"))]
    pub action: String,
    // 描述
    pub title: String,
}

// 添加或更新菜单
#[derive(Debug, Deserialize, Validate)]
pub struct InsertOrUpdateMenuPayload {
    // 父级
    #[validate(range(min = 1, message = "参数无效"))]
    pub pid: Option<i64>,

    // 路径
    #[validate(length(min = 1, message = "参数无效"))]
    pub path: String,

    // 名称
    #[validate(length(min = 1, message = "参数无效"))]
    pub name: String,

    // 组件
    #[validate(length(min = 1, message = "参数无效"))]
    pub component: Option<String>,

    // 重定向
    pub redirect: Option<String>,

    // 标题
    #[validate(length(min = 1, message = "参数无效"))]
    pub title: String,

    // 图标
    pub icon: Option<String>,

    // 顺序
    #[serde(rename = "orderNo")]
    #[validate(range(min = 0, max = 32766, message = "参数无效"))]
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
}

// 接口列表响应
#[derive(Debug, Serialize, FromRow)]
pub struct PermListResp {
    pub id: i64,
    // 菜单ID
    pub pid: i64,
    // 菜单名称
    pub menu: Option<String>,
    // 资源
    pub component: String,
    // 动作
    pub action: String,
    // 标题
    pub title: String,
}
