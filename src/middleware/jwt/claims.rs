use crate::{common::error::Error, config, state::user::UserState};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub state: UserState, // user id
    pub iat: usize,       // created time
    pub exp: usize,       // expired time
}

impl Claims {
    pub async fn new(state: UserState) -> Result<Self, Error> {
        let conf = config::get_config().await;

        // 创建时间
        let iat = Utc::now();
        // 过期时间
        let exp = iat + Duration::seconds(conf.jwt.get_expired());

        Ok(Self {
            state,
            iat: iat.timestamp() as usize,
            exp: exp.timestamp() as usize,
        })
    }
}
