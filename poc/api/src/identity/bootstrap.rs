use std::sync::Arc;

use anyhow::Result;
use cqrs_es::persist::PersistedEventStore;
use cqrs_es::{CqrsFramework, Query};
use mongodb::Client;
use postgres_es::{PostgresCqrs, PostgresEventRepository};
use sqlx::{Pool, Postgres};

use crate::identity::domain::account::Account;

use super::application::IdentityState;
use super::config::IdentityConfig;
use super::read::mongodb::accounts::AccountViewRepository;

#[derive(Clone)]
pub struct PostGresIdentityState {
    pub cqrs: Arc<PostgresCqrs<Account>>,
    pub account_views: Arc<AccountViewRepository>,
}
impl PostGresIdentityState {
    pub fn new(
        db_pool: Pool<Postgres>,
        mongo_client: Client,
        config: &IdentityConfig,
    ) -> Result<PostGresIdentityState> {
        let event_repository = PostgresEventRepository::new(db_pool);
        let view_repository = AccountViewRepository::new(
            mongo_client,
            config.read_store_database.clone().unwrap(),
            config.read_store_accounts_collection.clone().unwrap(),
        );
        let queries: Vec<Box<dyn Query<Account>>> = vec![Box::new(view_repository.clone())];
        Ok(Self {
            cqrs: Arc::new(CqrsFramework::new(
                PersistedEventStore::new_event_store(event_repository),
                queries,
                (),
            )),
            account_views: Arc::new(view_repository),
        })
    }
}

impl IdentityState<PersistedEventStore<PostgresEventRepository, Account>>
    for PostGresIdentityState
{
    fn cqrs(
        &self,
    ) -> Arc<CqrsFramework<Account, PersistedEventStore<PostgresEventRepository, Account>>> {
        self.cqrs.clone()
    }

    fn account_views(&self) -> Arc<dyn crate::identity::application::AccountViewRepository> {
        self.account_views.clone()
    }
}
