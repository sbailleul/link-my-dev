use std::sync::Arc;

use self::views::TeamView;
use anyhow::Result;
use cqrs_es::{CqrsFramework, EventStore};

use super::domain::team::Team;

pub mod views;

pub trait CollaborationState<ES: EventStore<Team>> {
    fn cqrs(&self) -> Arc<CqrsFramework<Team, ES>>;
    fn team_views(&self) -> Arc<dyn TeamViewRepository>;
}

#[async_trait]
pub trait TeamViewRepository {
    async fn get_all(&self) -> Result<Vec<TeamView>>;
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Option<TeamView>>;
}
