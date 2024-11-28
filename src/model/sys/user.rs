#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

/// 用户表名
pub const TABLE_SYS_USER: &str = "sys_user";

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    // 用户名
    pub username: String,
    // 密码
    pub password: String,
    // 手机号
    pub fullname: String,
    // 最后登录IP
    pub last_login_ip: Option<String>,
    // 最后登录时间
    pub last_login_at: Option<DateTime<Utc>>,
    // 账号状态
    pub is_active: bool,
    // 删除标记
    #[serde(skip)]
    pub is_deleted: bool,
    // 创建人
    pub created_by: i64,
    // 创建时间
    pub created_at: DateTime<Utc>,
    // 更新时间
    pub updated_at: DateTime<Utc>,
}
