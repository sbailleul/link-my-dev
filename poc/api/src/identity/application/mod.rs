use std::sync::Arc;

use self::views::AccountView;
use anyhow::Result;
use cqrs_es::{CqrsFramework, EventStore};

use super::domain::account::Account;

pub mod views;

pub trait IdentityState<ES: EventStore<Account>> {
    fn cqrs(&self) -> Arc<CqrsFramework<Account, ES>>;
    fn account_views(&self) -> Arc<dyn AccountViewRepository>;
}

#[async_trait]
pub trait AccountViewRepository {
    async fn get_all(&self) -> Result<Vec<AccountView>>;
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Option<AccountView>>;
}
