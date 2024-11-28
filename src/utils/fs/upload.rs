#![allow(dead_code)]

use crate::{common::error::Error, config, constants::UPLOADS_DIR};
use chrono::Local;
use std::{
    env::current_dir,
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};
use uuid::Uuid;

pub struct UploadFile;

impl UploadFile {
    // 转换为url连接
    pub async fn path_to_url(file_path: String) -> String {
        let conf = &config::get_config().await.server;

        format!("{}/{}", conf.get_domain(), file_path)
    }

    // 保存文件
    pub async fn save_file(filename: &str, data: &[u8]) -> Result<String, Error> {
        // 获取文件拓展名
        let ext = Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .ok_or(Error::Params("缺少文件扩展名"))?;

        // 新文件名
        let new_file_name = format!("{}.{}", Uuid::new_v4(), ext);

        // 创建文件夹
        let file_path = Self::create_dir_by_date().await?.join(new_file_name);

        // 写入文件
        File::create(&file_path).await?.write_all(data).await?;

        // 获取相对路径
        let src = file_path
            .strip_prefix(current_dir()?)
            .map_err(|_| Error::Unavailable)?
            .to_string_lossy()
            .to_string();

        Ok(src)
    }

    // 根据当前时间 年/月/日 自动生成文件夹
    pub async fn create_dir_by_date() -> Result<PathBuf, Error> {
        // 使用时间戳生成年/月/日的文件夹路径
        let now = Local::now();
        let year = now.format("%Y").to_string();
        let month = now.format("%m").to_string();
        let day = now.format("%d").to_string();

        // 获取主文件夹
        let static_dir = config::get_config().await.server.get_static_dir();
        let base_dir = Path::new(&static_dir)
            .join(UPLOADS_DIR)
            .join(year)
            .join(month)
            .join(day);

        // 异步创建目录（如果目录不存在则创建）
        create_dir_all(&base_dir).await?;

        Ok(base_dir)
    }

}
