use crate::{
    common::{
        error::Error,
        request::Pagination,
        response::{
            json::{Notify, QuantityList},
            Response,
        },
    },
    controller::car::brand::{
        delete, insert,
        select::{self, CarBrandQuery},
        update,
        upload::{self, UploadLogoResp},
        InsertOrUpdateCarBrand,
    },
    model::opt::car::brand::OptCarBrand,
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
        .route("/", get(get_car_brands))
        .route("/", post(add_car_brand))
        .route("/:id", put(update_car_brand))
        .route("/:id", delete(delete_car_brand))
        .route("/logo/:first_letter", post(upload_car_brand_logo))
        .with_state(state)
}

/*
**  车辆品牌列表
*/

async fn get_car_brands(
    State(app_state): State<AppState>,
    params: Pagination<CarBrandQuery>,
) -> Result<QuantityList<OptCarBrand>, Error> {
    Ok(select::query_car_brand_all(&app_state, params)
        .await?
        .into())
}

/*
**  添加车辆品牌
*/

async fn add_car_brand(
    State(app_state): State<AppState>,
    Json(payload): Json<InsertOrUpdateCarBrand>,
) -> Result<Notify, Error> {
    payload.validate()?;
    // 添加品牌
    Ok(insert::insert_car_brand(&app_state, payload).await?.into())
}

/*
**  更新车辆品牌
*/

async fn update_car_brand(
    State(app_state): State<AppState>,
    Path(brand_id): Path<i64>,
    Json(payload): Json<InsertOrUpdateCarBrand>,
) -> Result<Notify, Error> {
    payload.validate()?;

    Ok(
        update::update_car_brand_by_id(&app_state, brand_id, payload)
            .await?
            .into(),
    )
}

/*
**  删除车辆品牌
*/

async fn delete_car_brand(
    State(app_state): State<AppState>,
    Path(brand_id): Path<i64>,
) -> Result<Notify, Error> {
    Ok(delete::delete_car_brand_by_id(&app_state, brand_id)
        .await?
        .into())
}

/*
**  上传车辆logo
*/

async fn upload_car_brand_logo(
    Path(first_letter): Path<String>,
    multipart: Multipart,
) -> Result<Response<UploadLogoResp>, Error> {
    // 根据权限判断菜单
    Ok(upload::upload_logo(first_letter, multipart).await?.into())
}
