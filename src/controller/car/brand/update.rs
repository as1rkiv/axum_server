use super::InsertOrUpdateCarBrand;
use crate::{common::error::Error, model::opt::car::brand::TABLE_CAR_BRAND, state::AppState};
use tokio::fs;

// 更新品牌
pub async fn update_car_brand_by_id(
    app_state: &AppState,
    brand_id: i64,
    data: InsertOrUpdateCarBrand,
) -> Result<u64, Error> {
    // 查询当前logo
    let cur_query = format!(r#"SELECT logo FROM `{TABLE_CAR_BRAND}` WHERE `id` = ?"#);
    let current_logo_path: String = sqlx::query_scalar(&cur_query)
        .bind(brand_id)
        .fetch_one(app_state.mysql())
        .await?;

    // 删除原图片
    if current_logo_path != data.logo {
        fs::remove_file(current_logo_path).await.ok();
    }

    // 更新部门
    let query = format!(
        r#"UPDATE `{TABLE_CAR_BRAND}` SET 
        `name` = ?, `logo` = ?, `firstletter` = ?, 
        `country` = ? 
        WHERE `id` = ?"#
    );
    let result = sqlx::query(&query)
        .bind(data.name.trim())
        .bind(data.logo.trim())
        .bind(data.firstletter.trim())
        .bind(data.country.trim())
        .bind(brand_id)
        .execute(app_state.mysql())
        .await?;

    // 返回影响行数
    Ok(result.rows_affected())
}
