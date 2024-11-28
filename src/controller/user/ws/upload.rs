use crate::{common::error::Error, utils::fs::upload::UploadFile};
use axum::extract::Multipart;
use serde::Serialize;

// 上传文件成功响应
#[derive(Debug, Serialize)]
pub struct UploadChatDataResp {
    url: String,
}

// 上传文件
pub async fn upload_chat_files(mut multipart: Multipart) -> Result<UploadChatDataResp, Error> {
    // 匹配formData字段
    if let Some(field) = multipart.next_field().await? {
        if field.name() == Some("file") {
            // 获取文件名
            let filename = field
                .file_name()
                .ok_or(Error::Params("文件名称错误"))?
                .to_string();

            // 文件类型检查
            field.content_type().ok_or(Error::Params("文件类型错误"))?;

            // 处理文件
            let data = field.bytes().await?;
            let file_path = UploadFile::save_file(&filename, &data).await?;

            // 返回链接
            return Ok(UploadChatDataResp { url: file_path });
        }
    }

    Err(Error::Params("参数错误"))
}
