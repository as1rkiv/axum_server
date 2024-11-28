use crate::{
    common::{error::Error, request::Pagination},
    model::opt::car::{
        brand::TABLE_CAR_BRAND,
        series::{OptCarSeries, TABLE_CAR_SERIES},
    },
    state::AppState,
    store::mysql::QueryBuilderExt,
};
use serde::Deserialize;
use sqlx::QueryBuilder;
use validator::Validate;

// 汽车品牌列表查询项
#[derive(Debug, Deserialize, Validate)]
pub struct CarBrandSeriesQuery {
    pub brand_id: Option<String>,
    pub name: Option<String>,
    pub level: Option<String>,
    pub factory: Option<String>,
    pub power: Option<String>,
}

// 获取所有汽车品牌系列
pub async fn query_car_brand_series(
    app_state: &AppState,
    params: Pagination<CarBrandSeriesQuery>,
) -> Result<(i64, Vec<OptCarSeries>), Error> {
    // 查询数量
    let mut count_builder = QueryBuilder::select("COUNT(s.id) AS count", TABLE_CAR_SERIES, "s");

    count_builder
        .join(format!(
            "LEFT JOIN `{TABLE_CAR_BRAND}` b ON b.id = s.brand_id"
        ))
        .maybe()
        .and()
        .is_not_deleted("s");

    // 查询字段
    let mut query_builder =
        QueryBuilder::select("s.*, b.name AS brand_name", TABLE_CAR_SERIES, "s");

    query_builder
        .join(format!(
            "LEFT JOIN `{TABLE_CAR_BRAND}` b ON b.id = s.brand_id"
        ))
        .maybe()
        .and()
        .is_not_deleted("s");

    // 品牌ID
    if let Some(brand_id) = &params.get_params().brand_id {
        if !brand_id.is_empty() {
            if let Ok(id) = brand_id.parse::<i64>() {
                count_builder.and().eq("s.brand_id", id);
                query_builder.and().eq("s.brand_id", id);
            }
        }
    }

    // 系列名称
    if let Some(name) = &params.get_params().name {
        if !name.is_empty() {
            count_builder.and().like("s.name", name.trim());
            query_builder.and().like("s.name", name.trim());
        }
    }

    // 车型
    if let Some(level) = &params.get_params().level {
        if !level.is_empty() {
            count_builder.and().like("s.level", level.trim());
            query_builder.and().like("s.level", level.trim());
        }
    }

    // 制造商
    if let Some(factory) = &params.get_params().factory {
        if !factory.is_empty() {
            count_builder.and().like("s.factory", factory.trim());
            query_builder.and().like("s.factory", factory.trim());
        }
    }

    // 动力
    if let Some(power) = &params.get_params().power {
        if !power.is_empty() {
            count_builder.and().like("s.power", power.trim());
            query_builder.and().like("s.power", power.trim());
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
    let list: Vec<OptCarSeries> = query_builder
        .order_by("b.firstletter ASC, b.name ASC")
        .pagination(params.get_index(), params.get_size())
        .build_query_as()
        .fetch_all(app_state.mysql())
        .await?;

    Ok((count, list))
}
