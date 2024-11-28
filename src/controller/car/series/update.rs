use super::InsertOrUpdateCarBrandSeries;
use crate::{common::error::Error, model::opt::car::series::TABLE_CAR_SERIES, state::AppState};
use tokio::fs;

// 更新品牌系列
pub async fn update_car_brand_series_by_id(
    app_state: &AppState,
    series_id: i64,
    data: InsertOrUpdateCarBrandSeries,
) -> Result<u64, Error> {
    // 查询当前logo
    let cur_query = format!(r#"SELECT logo FROM `{TABLE_CAR_SERIES}` WHERE `id` = ?"#);
    let current_logo_path: String = sqlx::query_scalar(&cur_query)
        .bind(series_id)
        .fetch_one(app_state.mysql())
        .await?;

    // 删除原图片
    if current_logo_path != data.logo {
        fs::remove_file(current_logo_path).await.ok();
    }

    // 更新品牌
    let query = format!(
        r#"UPDATE `{TABLE_CAR_SERIES}` SET 
        `brand_id` = ?, `name` = ?, `logo` = ?, `level` = ?, 
        `factory` = ?, `power` = ? 
        WHERE `id` = ?"#
    );
    let result = sqlx::query(&query)
        .bind(data.brand_id)
        .bind(data.name.trim())
        .bind(data.logo.trim())
        .bind(data.level.trim())
        .bind(data.factory.trim())
        .bind(data.power.trim())
        .bind(series_id)
        .execute(app_state.mysql())
        .await?;

    // 返回影响行数
    Ok(result.rows_affected())
}
