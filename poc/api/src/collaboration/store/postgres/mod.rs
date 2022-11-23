pub mod queries;

use anyhow::Result;


use postgres_es::{
    default_postgress_pool, PostgresEventRepository, PostgresViewRepository,
};

use crate::collaboration::application::queries::TeamView;
use crate::collaboration::domain::team::Team;
use crate::Config;

pub async fn configure(
    config: &Config,
) -> Result<(
    PostgresEventRepository,
    PostgresViewRepository<TeamView, Team>,
)> {
    let pool = default_postgress_pool(
        config
            .collaboration_connection_string
            .clone()
            .unwrap()
            .as_str(),
    )
    .await;
    let event_store = PostgresEventRepository::new(pool.clone());
    let view_repository = PostgresViewRepository::new("team", pool);
    Ok((event_store, view_repository))
}
