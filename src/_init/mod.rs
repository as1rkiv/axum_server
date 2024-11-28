mod mysql;

use super::_init::mysql::init;
use crate::common::error::Error;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "app")]
pub struct Args {
    /// 初始化数据库
    #[arg(long)]
    initialize: Option<String>,
}

impl Args {
    // 从环境加载
    pub fn from_env() -> Self {
        Self::parse()
    }

    // 执行
    pub async fn exec(&self) -> Result<(), Error> {
        if let Some(db_conn_url) = &self.initialize {
            init(db_conn_url).await?
        }

        Ok(())
    }
}
