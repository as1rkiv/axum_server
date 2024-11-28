use crate::{common::error::Error, model::opt::car::brand::TABLE_CAR_BRAND, state::AppState};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CarBrandOption {
    #[serde(rename = "value")]
    id: i64,
    #[serde(rename = "label")]
    name: String,
}

// 获取汽车品牌选项
pub async fn query_car_brand_option(app_state: &AppState) -> Result<Vec<CarBrandOption>, Error> {
    // 查询字段
    let query = format!(
        r#"SELECT `id`, `name` FROM `{TABLE_CAR_BRAND}` WHERE `is_deleted` = 0 ORDER BY `firstletter` ASC, `name` ASC;"#
    );

    Ok(sqlx::query_as(&query).fetch_all(app_state.mysql()).await?)
}
