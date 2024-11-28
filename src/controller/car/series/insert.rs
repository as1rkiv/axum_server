use super::InsertOrUpdateCarBrandSeries;
use crate::{common::error::Error, model::opt::car::series::TABLE_CAR_SERIES, state::AppState};

// 添加品牌系列
pub async fn insert_car_brand_series(
    app_state: &AppState,
    data: InsertOrUpdateCarBrandSeries,
) -> Result<u64, Error> {
    let query = format!(
        r#"INSERT INTO `{TABLE_CAR_SERIES}` ( 
        `brand_id`, `name`, `logo`, `level`, `factory`, `power` 
        ) VALUES (?, ?, ?, ?, ?, ?);"#
    );
    let result = sqlx::query(&query)
        .bind(data.brand_id)
        .bind(data.name.trim())
        .bind(data.logo.trim())
        .bind(data.level.trim())
        .bind(data.factory.trim())
        .bind(data.power.trim())
        .execute(app_state.mysql())
        .await?;

    Ok(result.rows_affected())
}
