use std::sync::Arc;

use cqrs_es::{CqrsFramework, EventStore};

use self::views::TeamView;

use super::domain::team::Team;

pub mod queries;
mod views;

pub trait CollaborationState<ES: EventStore<Team>> {
    fn cqrs(&self) -> Arc<CqrsFramework<Team, ES>>;
}

pub trait TeamViewRepository {
    fn get_all() -> Vec<TeamView>;
}
