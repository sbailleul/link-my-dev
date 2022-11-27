use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TeamEvent {
    TeamCreated { id: Uuid, name: String },
    NameChanged { id: Uuid, name: String }
}

impl DomainEvent for TeamEvent {
    fn event_type(&self) -> String {
        let event_type = match self {
            TeamEvent::TeamCreated { .. } => "TeamCreated",
            TeamEvent::NameChanged { .. } => "NameChanged",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
