#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RabbitmqConfig {
    host: String,
    port: i64,
    username: String,
    password: String,
}

impl RabbitmqConfig {
    // rabbitmq 连接
    pub fn get_conn_url(&self) -> String {
        format!(
            "amqp://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
