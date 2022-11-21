use cqrs_es::{EventEnvelope, Query, View};
use cqrs_es::persist::GenericQuery;
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};

use crate::collaboration::domain::events::TeamEvent;
use crate::collaboration::domain::events::TeamEvent::TeamCreated;
use crate::collaboration::domain::team::Team;

pub struct SimpleLoggingQuery {}

#[async_trait]
impl Query<Team> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Team>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TeamView {
    id: String,
    name: String,
}


impl View<Team> for TeamView {
    fn update(&mut self, event: &EventEnvelope<Team>) {
        match &event.payload {
            TeamCreated { name, id    } => {
                self.name = name.clone();
                self.id = id.clone();
            }
        }
    }
}