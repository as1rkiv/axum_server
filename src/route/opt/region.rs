use crate::{
    common::{
        error::Error,
        request::Pagination,
        response::json::{Notify, QuantityList, SingleList},
    },
    controller::region::{delete, insert, options, select, update, InsertOrUpdateOptRegion},
    model::opt::region::OptRegion,
    state::AppState,
};
use axum::{
    extract::{Json, Path, State},
    routing::{delete, get, post, put},
    Router,
};
use validator::Validate;

// 品牌路由
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_region_list))
        .route("/", post(add_region))
        .route("/:id", put(update_region))
        .route("/:id", delete(delete_region))
        .route("/options", get(get_region_options))
        .with_state(state)
}

/*
**  行政地区连级选项
*/

async fn get_region_options(
    State(app_state): State<AppState>,
) -> Result<SingleList<options::OptRegionOptions>, Error> {
    Ok(options::query_region_options(&app_state).await?.into())
}

/*
**  行政地区列表
*/

async fn get_region_list(
    State(app_state): State<AppState>,
    params: Pagination<select::RegionQuery>,
) -> Result<QuantityList<OptRegion>, Error> {
    Ok(select::query_region_list(&app_state, params).await?.into())
}

/*
**  添加行政地区
*/

async fn add_region(
    State(app_state): State<AppState>,
    Json(payload): Json<InsertOrUpdateOptRegion>,
) -> Result<Notify, Error> {
    payload.validate()?;
    Ok(insert::insert_region(&app_state, payload).await?.into())
}

/*
**  更新行政地区
*/

async fn update_region(
    State(app_state): State<AppState>,
    Path(region_id): Path<i64>,
    Json(payload): Json<InsertOrUpdateOptRegion>,
) -> Result<Notify, Error> {
    payload.validate()?;

    Ok(update::update_region_by_id(&app_state, region_id, payload)
        .await?
        .into())
}

/*
**  删除行政地区
*/

async fn delete_region(
    State(app_state): State<AppState>,
    Path(region_id): Path<i64>,
) -> Result<Notify, Error> {
    Ok(delete::delete_region_by_id(&app_state, region_id)
        .await?
        .into())
}
