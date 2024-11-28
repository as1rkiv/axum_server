#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    host: String,
    port: i64,
    username: String,
    password: String,
}

impl RedisConfig {
    // redis连接
    pub fn get_conn_url(&self, index: u32) -> String {
        format!(
            "redis://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, index
        )
    }
}
