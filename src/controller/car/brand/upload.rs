use super::super::{STATIC_CAR_BRAND_DIR, STATIC_CAR_DIR};
use crate::{common::error::Error, config};
use axum::extract::Multipart;
use serde::Serialize;
use std::{env::current_dir, ffi::OsStr, path::Path};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
};
use uuid::Uuid;

// 上传文件成功响应
#[derive(Debug, Serialize)]
pub struct UploadLogoResp {
    url: String,
}

// 上传文件
pub async fn upload_logo(
    first_letter: String,
    mut multipart: Multipart,
) -> Result<UploadLogoResp, Error> {
    // 匹配formData字段
    if let Some(field) = multipart.next_field().await? {
        if field.name() == Some("logo") {
            // 获取文件名
            let filename = field
                .file_name()
                .ok_or(Error::Params("文件名称错误"))?
                .to_string();

            // 文件类型检查
            field.content_type().ok_or(Error::Params("文件类型错误"))?;

            // 处理文件
            let data = field.bytes().await?;

            // 获取主文件夹
            let static_dir = config::get_config().await.server.get_static_dir();
            let base_dir = Path::new(&static_dir)
                .join(STATIC_CAR_DIR)
                .join(STATIC_CAR_BRAND_DIR)
                .join(first_letter);

            // 异步创建目录（如果目录不存在则创建）
            create_dir_all(&base_dir).await?;

            // 获取文件拓展名
            let ext = Path::new(&filename)
                .extension()
                .and_then(OsStr::to_str)
                .ok_or(Error::Params("缺少文件扩展名"))?;

            // 新文件名
            let new_file_name = format!("{}.{}", Uuid::new_v4(), ext);
            let file_path = base_dir.join(&new_file_name);

            // 写入文件
            File::create(&file_path).await?.write_all(&data).await?;

            // 获取相对路径
            let src = file_path
                .strip_prefix(current_dir()?)
                .map_err(|_| Error::Unavailable)?
                .to_string_lossy()
                .to_string();

            // 返回链接
            return Ok(UploadLogoResp { url: src });
        }
    }

    Err(Error::Params("参数错误"))
}
