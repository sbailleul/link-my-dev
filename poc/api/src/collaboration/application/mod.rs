use std::sync::Arc;

use cqrs_es::{CqrsFramework, EventStore};
use anyhow::Result;
use self::views::TeamView;

use super::domain::team::Team;

pub mod queries;
pub mod views;

pub trait CollaborationState<ES: EventStore<Team>> {
    fn cqrs(&self) -> Arc<CqrsFramework<Team, ES>>;
    fn team_views(&self) -> Arc<dyn TeamViewRepository>;
}

#[async_trait]
pub trait TeamViewRepository {
    async fn get_all(&self) -> Result<Vec<TeamView>>;
}
