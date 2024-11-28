use super::InsertOrUpdateCarBrand;
use crate::{common::error::Error, model::opt::car::brand::TABLE_CAR_BRAND, state::AppState};

// 添加品牌
pub async fn insert_car_brand(
    app_state: &AppState,
    data: InsertOrUpdateCarBrand,
) -> Result<u64, Error> {
    let query = format!(
        r#"INSERT INTO `{TABLE_CAR_BRAND}` ( 
            `name`, `logo`, `firstletter`,  `country`
        ) VALUES (?, ?, ?, ?)"#
    );
    let result = sqlx::query(&query)
        .bind(data.name.trim())
        .bind(data.logo.trim())
        .bind(data.firstletter.trim())
        .bind(data.country.trim())
        .execute(app_state.mysql())
        .await?;

    Ok(result.rows_affected())
}
