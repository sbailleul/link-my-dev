use std::sync::Arc;

use anyhow::Result;
use cqrs_es::persist::{PersistedEventStore, ViewRepository};
use cqrs_es::{CqrsFramework, EventStore, Query};
use postgres_es::{PostgresCqrs, PostgresEventRepository, PostgresViewRepository};
use sqlx::{Pool, Postgres, pool};


use crate::collaboration::application::queries::{SimpleLoggingQuery};
use crate::collaboration::domain::team::Team;
use crate::Config;

use super::application::CollaborationState;

#[derive(Clone)]
pub struct  PostGresCollaborationState{
    pub cqrs: Arc<PostgresCqrs<Team>>,
}
impl PostGresCollaborationState {
    pub fn new(db_pool: Pool<Postgres>) -> Result<PostGresCollaborationState> {
        let event_repository = PostgresEventRepository::new(db_pool.clone());
        let simple_query = SimpleLoggingQuery {};
        let queries: Vec<Box<dyn Query<Team>>> =
            vec![Box::new(simple_query)];
        Ok(Self {
            cqrs: Arc::new(CqrsFramework::new(
                PersistedEventStore::new_event_store(event_repository),
                queries,
                (),
            ))
        })
    }
}

impl CollaborationState<PersistedEventStore<PostgresEventRepository, Team>> for PostGresCollaborationState{
    fn cqrs(&self) -> Arc<CqrsFramework<Team, PersistedEventStore<PostgresEventRepository, Team>>> {
        self.cqrs.clone()
    }
}