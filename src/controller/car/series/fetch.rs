use crate::{
    common::error::Error,
    model::car::{
        brand::{CarBrand, TABLE_CAR_BRAND},
        series::TABLE_CAR_BRAND_SERIES,
    },
    state::AppState,
};
use serde::Deserialize;
use std::path::Path;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CarSeries {
    seriesname: String,
    levelname: String,
    fueltypedetailname: String,
    fctname: String,
    pnglogo: String,
}

#[derive(Debug, Deserialize)]
struct SearchOption {
    envalue: i32,
    key: String,
    name: String,
    value: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct Res {
    searchoption1: Vec<SearchOption>,
    serieslist: Vec<CarSeries>,
}

#[derive(Debug, Deserialize)]
struct Resp {
    returncode: i32,
    message: String,
    result: Res,
}

pub async fn get_brand_series(app_state: &AppState) -> Result<(), Error> {
    //     todo!()

    let brands = get_brands().await?;

    for brand in brands.iter() {
        let req = format!(
            "
    https://car-web-api.autohome.com.cn/car/series/getserieslistbybrandid?brandid={}",
            &brand.id
        );

        // 请求结果
        let res = reqwest::get(req).await?.bytes().await?;
        let result: Resp = serde_json::from_slice(&res)?;

        // 查询数据库中的ID
        let query = format!("SELECT `id` FROM `{TABLE_CAR_BRAND}` WHERE `name` = ?");
        let brand_id: i64 = sqlx::query_scalar(&query)
            .bind(&brand.name)
            .fetch_one(app_state.mysql())
            .await?;

        //循环写入
        for series in result.result.serieslist.iter() {
            let file_path = if !&series.pnglogo.is_empty() {
                let _img = reqwest::get(&series.pnglogo).await?;

                // 获取 Content-Type
                let header = _img.headers().clone();

                let content_type = header
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("");

                let bytes = &_img.bytes().await?;

                // 根据 Content-Type 获取扩展名
                let ext = match content_type {
                    "image/jpeg" => "jpg",
                    "image/png" => "png",
                    "image/gif" => "gif",
                    "image/webp" => "webp",
                    _ => "bin", // 如果未识别类型，可以使用默认扩展名
                };

                // 生成文件路径（使用品牌 ID 作为文件名）
                let file_path =
                    format!("static/car/series/{}/{}.{}", &brand_id, Uuid::new_v4(), ext);
                let path = Path::new(&file_path);

                // 创建目录（如果尚未创建）
                if let Some(parent) = path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                // 写入文件
                let mut file = File::create(&file_path).await?;
                file.write_all(&bytes).await?;

                file_path
            } else {
                String::new()
            };

            // 写入数据库
            let insert_query = format!(
                r#"INSERT INTO `{TABLE_CAR_BRAND_SERIES}` ( 
                `brand_id`, `name`, `logo`, `level`, `factory`, `power` 
                ) VALUES (?, ?, ?, ?, ?, ?);"#
            );
            sqlx::query(&insert_query)
                .bind(&brand_id)
                .bind(&series.seriesname)
                .bind(file_path)
                .bind(&series.levelname)
                .bind(&series.fctname)
                .bind(&series.fueltypedetailname)
                .execute(app_state.mysql())
                .await?;
        }
    }
    Ok(())
}

async fn get_brands() -> Result<Vec<CarBrand>, Error> {
    let mut file = File::open("brand.json").await?;
    let mut content = String::new();

    file.read_to_string(&mut content).await?;

    Ok(serde_json::from_str(&content)?)
}
