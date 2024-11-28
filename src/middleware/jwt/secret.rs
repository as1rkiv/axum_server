use crate::config;
use jsonwebtoken::{DecodingKey, EncodingKey};
use tokio::sync::OnceCell;

static SECRET: OnceCell<Secret> = OnceCell::const_new();

pub async fn get_secret() -> &'static Secret {
    SECRET
        .get_or_init(|| async {
            let secret = config::get_config().await.jwt.get_secret();
            Secret::new(secret.as_bytes())
        })
        .await
}

pub struct Secret {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Secret {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
