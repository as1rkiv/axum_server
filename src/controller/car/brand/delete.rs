use crate::{common::error::Error, model::opt::car::brand::TABLE_CAR_BRAND, state::AppState};

// 软删除品牌
pub async fn delete_car_brand_by_id(app_state: &AppState, brand_id: i64) -> Result<u64, Error> {
    let query = format!(r#"UPDATE `{TABLE_CAR_BRAND}` SET `is_deleted` = 1 WHERE `id` = ?"#);
    let result = sqlx::query(&query)
        .bind(brand_id)
        .execute(app_state.mysql())
        .await?;

    Ok(result.rows_affected())
}
