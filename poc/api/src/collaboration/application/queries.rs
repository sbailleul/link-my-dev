use cqrs_es::{EventEnvelope, Query, View};


use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::collaboration::domain::events::TeamEvent::{TeamCreated, NameChanged};
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


