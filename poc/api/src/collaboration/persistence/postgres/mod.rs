
use anyhow::Result;

use postgres_es::{default_postgress_pool, PostgresEventRepository, PostgresViewRepository};
use sqlx::{Pool, Postgres};

use crate::{Config, collaboration::config::CollaborationConfig};

pub async fn get_pool(config: &CollaborationConfig) -> Result<Pool<Postgres>> {
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
