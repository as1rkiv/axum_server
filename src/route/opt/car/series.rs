use crate::{
    common::{
        error::Error,
        request::Pagination,
        response::{
            json::{Notify, QuantityList, SingleList},
            Response,
        },
    },
    controller::car::{
        brand::option::{self, CarBrandOption},
        series::{
            delete, insert,
            select::{self, CarBrandSeriesQuery},
            update,
            upload::{self, UploadImageResp},
            InsertOrUpdateCarBrandSeries,
        },
    },
    model::opt::car::series::OptCarSeries,
    state::AppState,
};
use axum::{
    extract::{Json, Multipart, Path, State},
    routing::{delete, get, post, put},
    Router,
};
use validator::Validate;

// 品牌路由
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_car_brand_series))
        .route("/", post(add_car_brand_series))
        .route("/:id", put(update_car_brand_series))
        .route("/:id", delete(delete_car_brand_series))
        .route("/brands", get(get_car_brand_option))
        .route("/img/:brand_id", post(upload_car_brand_series_img))
        .with_state(state)
}

/*
**  车辆品牌选项
*/

async fn get_car_brand_option(
    State(app_state): State<AppState>,
) -> Result<SingleList<CarBrandOption>, Error> {
    Ok(option::query_car_brand_option(&app_state).await?.into())
}

/*
**  车辆系列列表
*/

async fn get_car_brand_series(
    State(app_state): State<AppState>,
    params: Pagination<CarBrandSeriesQuery>,
) -> Result<QuantityList<OptCarSeries>, Error> {
    Ok(select::query_car_brand_series(&app_state, params)
        .await?
        .into())
}

/*
**  添加车辆系列
*/

async fn add_car_brand_series(
    State(app_state): State<AppState>,
    Json(payload): Json<InsertOrUpdateCarBrandSeries>,
) -> Result<Notify, Error> {
    payload.validate()?;
    Ok(insert::insert_car_brand_series(&app_state, payload)
        .await?
        .into())
}

/*
**  更新车辆系列
*/

async fn update_car_brand_series(
    State(app_state): State<AppState>,
    Path(series_id): Path<i64>,
    Json(payload): Json<InsertOrUpdateCarBrandSeries>,
) -> Result<Notify, Error> {
    payload.validate()?;

    Ok(
        update::update_car_brand_series_by_id(&app_state, series_id, payload)
            .await?
            .into(),
    )
}

/*
**  删除车辆系列
*/

async fn delete_car_brand_series(
    State(app_state): State<AppState>,
    Path(series_id): Path<i64>,
) -> Result<Notify, Error> {
    Ok(delete::delete_car_brand_series_by_id(&app_state, series_id)
        .await?
        .into())
}

/*
**  上传车辆系列图片
*/

async fn upload_car_brand_series_img(
    Path(brand_id): Path<String>,
    multipart: Multipart,
) -> Result<Response<UploadImageResp>, Error> {
    // 根据权限判断菜单
    Ok(upload::upload_image(brand_id, multipart).await?.into())
}
