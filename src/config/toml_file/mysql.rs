#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MysqlConfig {
    host: String,
    port: i64,
    username: String,     // root 用户
    password: String,     // root 密码
    database: String,     // 库名
    charset: String,      // 字符集
    collate: String,      // 校对规则
    max_connections: u32, // 最大连接数
    min_connections: u32, // 最小连接数
    acquire_timeout: u64, // 超时时间(s)
}

impl MysqlConfig {
    // 数据库连接
    pub fn get_conn_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}?charset={}&loc=Local",
            self.username, self.password, self.host, self.port, self.database, self.charset
        )
    }

    // 用户名
    pub fn get_username(&self) -> &str {
        &self.username
    }

    // 密码
    pub fn get_password(&self) -> &str {
        &self.password
    }

    // 库名
    pub fn get_database(&self) -> &str {
        &self.database
    }

    // 字符集
    pub fn get_charset(&self) -> &str {
        &self.charset
    }

    // 校对规则
    pub fn get_collate(&self) -> &str {
        &self.collate
    }

    // 连接池连接数 - 最大
    pub fn get_max_connections(&self) -> u32 {
        self.max_connections
    }

    // 连接池连接数 - 最小
    pub fn get_min_connections(&self) -> u32 {
        self.min_connections
    }

    // 连接超时时间(s)
    pub fn get_acquire_timeout(&self) -> u64 {
        self.acquire_timeout
    }
}
