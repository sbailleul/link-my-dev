use std::sync::Arc;

use anyhow::Result;
use cqrs_es::persist::PersistedEventStore;
use cqrs_es::{CqrsFramework, Query};
use mongodb::Client;
use postgres_es::{PostgresCqrs, PostgresEventRepository};
use sqlx::{Pool, Postgres};

use crate::collaboration::domain::team::Team;

use super::application::CollaborationState;
use super::config::CollaborationConfig;
use super::read::mongodb::teams::TeamViewRepository;

#[derive(Clone)]
pub struct PostGresCollaborationState {
    pub cqrs: Arc<PostgresCqrs<Team>>,
    pub team_views: Arc<TeamViewRepository>,
}
impl PostGresCollaborationState {
    pub fn new(
        db_pool: Pool<Postgres>,
        mongo_client: Client,
        config: &CollaborationConfig,
    ) -> Result<PostGresCollaborationState> {
        let event_repository = PostgresEventRepository::new(db_pool);
        let view_repository = TeamViewRepository::new(
            mongo_client,
            config.read_store_database.clone().unwrap(),
            config.read_store_teams_collection.clone().unwrap(),
        );
        let queries: Vec<Box<dyn Query<Team>>> = vec![Box::new(view_repository.clone())];
        Ok(Self {
            cqrs: Arc::new(CqrsFramework::new(
                PersistedEventStore::new_event_store(event_repository),
                queries,
                (),
            )),
            team_views: Arc::new(view_repository),
        })
    }
}

impl CollaborationState<PersistedEventStore<PostgresEventRepository, Team>>
    for PostGresCollaborationState
{
    fn cqrs(&self) -> Arc<CqrsFramework<Team, PersistedEventStore<PostgresEventRepository, Team>>> {
        self.cqrs.clone()
    }

    fn team_views(&self) -> Arc<dyn crate::collaboration::application::TeamViewRepository> {
        self.team_views.clone()
    }
}
