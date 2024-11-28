use super::REGION_CACHE_KEY;
use crate::{
    common::error::Error,
    model::opt::region::TABLE_OPT_REGION,
    state::AppState,
    store::mysql::QueryBuilderExt,
    utils::recursion::{Recursion, Recursive},
};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sqlx::{query_builder::QueryBuilder, FromRow};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OptRegionOptions {
    #[serde(rename = "value")]
    pub id: i64,

    // 父地区ID
    #[serde(skip)]
    pub pid: Option<i64>,

    // 地区名称
    #[serde(rename = "label")]
    pub name: String,

    // 子地区 - 跳过 sqlx 映射
    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Self>>,
}

// 递归插入
impl Recursive for OptRegionOptions {
    fn get_id(&self) -> i64 {
        self.id
    }
    fn get_pid(&self) -> Option<i64> {
        self.pid
    }
    fn get_children_mut(&mut self) -> &mut Option<Vec<Self>> {
        &mut self.children
    }
}

// 获取区域选项
pub async fn query_region_options(app_state: &AppState) -> Result<Vec<OptRegionOptions>, Error> {
    // 获取redis连接
    let mut redis_conn = app_state.redis_data().get().await?;

    // 检查缓存
    if let Some(cached_data) = redis_conn
        .get::<_, Option<Vec<u8>>>(REGION_CACHE_KEY)
        .await?
    {
        tracing::info!("Redis 数据缓存命中: {}", REGION_CACHE_KEY);
        return Ok(serde_json::from_slice(&cached_data)?);
    }

    // 构建查询
    let options: Vec<OptRegionOptions> =
        QueryBuilder::select("r.id, r.pid, r.name", TABLE_OPT_REGION, "r")
            .maybe()
            .and()
            .is_not_deleted("r")
            .build_query_as()
            .fetch_all(app_state.mysql())
            .await?;

    // 递归插入
    let data = Recursion::insert_childrens(options)?;

    // 缓存数据
    redis_conn
        .set::<_, _, ()>(REGION_CACHE_KEY, serde_json::to_vec(&data)?)
        .await
        .map_err(|e| tracing::error!("Redis 数据缓存失败: {}", e))
        .ok();

    Ok(data)
}
