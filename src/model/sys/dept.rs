#![allow(dead_code)]

use crate::utils::recursion::Recursive;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 部门表名
pub const TABLE_SYS_DEPT: &str = "sys_dept";

// 部门
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Dept {
    pub id: i64,

    // 父部门ID
    pub pid: Option<i64>,

    // 名称
    pub name: String,

    // 描述
    pub desc: String,

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

    // 子部门 - 跳过 sqlx 映射
    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Self>>,
}

// 部门递归插入
impl Recursive for Dept {
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
