use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TeamEvent {
    TeamCreated { id: String, name: String }
}

impl DomainEvent for TeamEvent {
    fn event_type(&self) -> String {
        let event_type = match self {
            TeamEvent::TeamCreated { .. } => "AccountOpened"
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}