use crate::{common::error::Error, model::opt::car::series::TABLE_CAR_SERIES, state::AppState};

// 软删除车系
pub async fn delete_car_brand_series_by_id(
    app_state: &AppState,
    series_id: i64,
) -> Result<u64, Error> {
    let query = format!(r#"UPDATE `{TABLE_CAR_SERIES}` SET `is_deleted` = 1 WHERE `id` = ?"#);
    let result = sqlx::query(&query)
        .bind(series_id)
        .execute(app_state.mysql())
        .await?;

    Ok(result.rows_affected())
}
