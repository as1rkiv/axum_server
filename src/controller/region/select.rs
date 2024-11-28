use crate::{
    common::{error::Error, request::Pagination},
    model::opt::region::{OptRegion, TABLE_OPT_REGION},
    state::AppState,
    store::mysql::QueryBuilderExt,
};
use serde::Deserialize;
use sqlx::QueryBuilder;
use validator::Validate;

// 行政地区列表查询项
#[derive(Debug, Deserialize, Validate)]
pub struct RegionQuery {
    pub pid: Option<String>,
}

// 获取行政地区列表
pub async fn query_region_list(
    app_state: &AppState,
    params: Pagination<RegionQuery>,
) -> Result<(i64, Vec<OptRegion>), Error> {
    // 查询数量
    let mut count_builder = QueryBuilder::select("COUNT(r.id) AS count", TABLE_OPT_REGION, "r");
    count_builder.maybe().and().is_not_deleted("r");

    // 查询字段
    let mut query_builder = QueryBuilder::select("r.*", TABLE_OPT_REGION, "r");
    query_builder.maybe().and().is_not_deleted("r");

    // 地区ID
    if let Some(pid) = &params.get_params().pid {
        if !pid.is_empty() {
            count_builder
                .and()
                .eq("r.id", pid.trim())
                .or()
                .eq("r.pid", pid.trim());
            query_builder
                .and()
                .eq("r.id", pid.trim())
                .or()
                .eq("r.pid", pid.trim());
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

    // 获取地区
    let list: Vec<OptRegion> = query_builder
        .order_by("r.id ASC, r.code ASC")
        .pagination(params.get_index(), params.get_size())
        .build_query_as()
        .fetch_all(app_state.mysql())
        .await?;

    Ok((count, list))
}
