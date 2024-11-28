use crate::middleware::privilege::data::DataScope;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

/// 角色表名
pub const TABLE_SYS_ROLE: &str = "sys_role";

// 用户角色
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Role {
    pub id: i64,
    // 角色名
    pub name: String,
    // 数据访问范围
    pub data_scope: DataScope,
    // 角色描述
    pub desc: Option<String>,
    // 创建人
    pub created_by: i64,
    // 创建时间
    pub created_at: DateTime<Utc>,
    // 更新时间
    pub updated_at: DateTime<Utc>,
}
