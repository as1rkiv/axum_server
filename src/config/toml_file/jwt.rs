#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    secret: String, // 密钥
    expired: i64,   // 过期时间(sec)
}

impl JwtConfig {
    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    pub fn get_expired(&self) -> i64 {
        self.expired
    }
}
