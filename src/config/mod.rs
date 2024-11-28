mod toml_file;

use crate::{common::error::Error, constants::CONFIG_FILE};
use serde::Deserialize;
use tokio::{fs::File, io::AsyncReadExt, sync::OnceCell};

use toml_file::{JwtConfig, LogConfig, MysqlConfig, RabbitmqConfig, RedisConfig, ServerConfig};

// 配置文件实例
static CONFIG_INSTANCE: OnceCell<Config> = OnceCell::const_new();

// 获取配置文件
pub async fn get_config() -> &'static Config {
    CONFIG_INSTANCE
        .get_or_init(|| async { Config::load_from_file().await.unwrap() })
        .await
}

#[derive(Debug, Deserialize)]
pub struct Config {
    // 服务器配置
    pub server: ServerConfig,
    // jwt配置
    pub jwt: JwtConfig,
    // 日志配置
    pub log: LogConfig,
    // mysql配置
    pub mysql: MysqlConfig,
    // redis配置
    pub redis: RedisConfig,
    // rabbitmq配置
    pub rabbitmq: RabbitmqConfig,
}

impl Config {
    async fn load_from_file() -> Result<Self, Error> {
        // 打开配置文件
        let mut file = File::open(CONFIG_FILE).await.map_err(|e| {
            tracing::error!("配置文件错误: {}", e);

            Error::Unavailable
        })?;

        // 读取配置文件
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.map_err(|e| {
            tracing::error!("配置文件错误: {}", e);

            Error::Unavailable
        })?;

        // 解析配置文件
        let config = toml::from_str(&contents).map_err(|e| {
            tracing::error!("配置文件错误: {}", e);

            Error::Unavailable
        })?;

        Ok(config)
    }
}
