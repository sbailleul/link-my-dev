use std::sync::Arc;

use anyhow::Result;
use cqrs_es::persist::{PersistedEventRepository, PersistedEventStore, ViewRepository};
use cqrs_es::{AggregateContext, CqrsFramework, EventStore, Query};
use postgres_es::{PostgresCqrs, PostgresEventRepository, PostgresViewRepository};
use sqlx::{Pool, Postgres};

use crate::collaboration::application::queries::{SimpleLoggingQuery, TeamView};
use crate::collaboration::domain::team::Team;
use crate::collaboration::store::postgres::configure;
use crate::collaboration::store::postgres::queries::TeamQuery;
use crate::Config;

pub struct CollaborationState {
    pub cqrs: Arc<PostgresCqrs<Team>>,
    pub views: Arc<PostgresViewRepository<TeamView, Team>>,
}

impl CollaborationState {
    pub async fn new(config: &Config) -> Result<CollaborationState> {
        let (repo, view_repository) = configure(config).await?;
        let simple_query = SimpleLoggingQuery {};
        let view_repository = Arc::new(view_repository);
        let mut account_query = TeamQuery::new(view_repository.clone());

        account_query.use_error_handler(Box::new(|e| println!("{}", e)));

        let queries: Vec<Box<dyn Query<Team>>> =
            vec![Box::new(simple_query), Box::new(account_query)];
        Ok(Self {
            cqrs: Arc::new(CqrsFramework::new(
                PersistedEventStore::new_event_store(repo),
                queries,
                (),
            )),
            views: view_repository.clone(),
        })
    }
}
