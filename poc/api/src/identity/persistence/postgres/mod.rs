use anyhow::Result;

use postgres_es::default_postgress_pool;
use sqlx::{Pool, Postgres};

use crate::identity::config::IdentityConfig;

pub async fn get_pool(config: &IdentityConfig) -> Result<Pool<Postgres>> {
    let pool = default_postgress_pool(
        config
            .event_store_connection_string
            .clone()
            .unwrap()
            .as_str(),
    )
    .await;
    anyhow::Ok(pool)
}
