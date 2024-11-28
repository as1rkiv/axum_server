#![allow(dead_code)]

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    host: String,
    port: i64,
    domain: String,
    protocol: String,         // http | https 协议
    tls_cert: Option<String>, // SSL 证书
    tls_key: Option<String>,  // SSL 密钥
    static_dir: String,       // 静态文件目录
}

impl ServerConfig {
    // 获取完整服务域名
    pub fn get_domain(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.domain, self.port)
    }

    // 获取监听地址
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    // 获取端口
    pub fn get_port(&self) -> i64 {
        self.port
    }

    // 获取静态服务文件夹
    pub fn get_static_dir(&self) -> String {
        let path = PathBuf::from(&self.static_dir);

        // 标准化路径（去除尾部的`/` 或 `\`）
        let normalized_path = path.canonicalize().unwrap_or(path);

        // 自动转换为合法字符串路径
        normalized_path.to_string_lossy().to_string()
    }
}
