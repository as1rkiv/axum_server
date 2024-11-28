use crate::{
    common::{error::Error, request::Pagination},
    model::opt::car::brand::{OptCarBrand, TABLE_CAR_BRAND},
    state::AppState,
    store::mysql::QueryBuilderExt,
};
use serde::Deserialize;
use sqlx::QueryBuilder;
use validator::Validate;

// 汽车品牌列表查询项
#[derive(Debug, Deserialize, Validate)]
pub struct CarBrandQuery {
    pub name: Option<String>,
}

// 获取所有汽车品牌
pub async fn query_car_brand_all(
    app_state: &AppState,
    params: Pagination<CarBrandQuery>,
) -> Result<(i64, Vec<OptCarBrand>), Error> {
    // 查询数量
    let mut count_builder = QueryBuilder::select("COUNT(b.id) AS count", TABLE_CAR_BRAND, "b");

    // 查询字段
    let mut query_builder = QueryBuilder::select("b.*", TABLE_CAR_BRAND, "b");

    count_builder.maybe().and().is_not_deleted("b");
    query_builder.maybe().and().is_not_deleted("b");

    // 品牌名
    if let Some(name) = &params.get_params().name {
        if !name.is_empty() {
            count_builder.and().like("b.name", name.trim());
            query_builder.and().like("b.name", name.trim());
        }
    }

    // 查询数量
    let count: i64 = match count_builder
        .build_query_scalar()
        .fetch_one(app_state.mysql())
        .await?
    {
        0 => return Ok((0, vec![])), // 数量为0时直接返回空列表
        count => count,
    };

    // 获取所有品牌
    let list: Vec<OptCarBrand> = query_builder
        .order_by("b.firstletter ASC, b.name ASC")
        .pagination(params.get_index(), params.get_size())
        .build_query_as()
        .fetch_all(app_state.mysql())
        .await?;

    Ok((count, list))
}
