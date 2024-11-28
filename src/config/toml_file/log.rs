#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    level: String,
    max_files: usize,
    rotation: String,
}

impl LogConfig {
    // 获取日志过滤级别
    pub fn get_level(&self) -> &str {
        &self.level
    }
    // 日志文件最大保留数量
    pub fn get_max_files(&self) -> usize {
        self.max_files
    }
    // 文件轮转 天|小时|分钟
    pub fn get_rotation(&self) -> &str {
        &self.rotation
    }
}
