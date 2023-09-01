use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountEvent {
    AccountRegistered { id: String, email: String },
}

impl DomainEvent for AccountEvent {
    fn event_type(&self) -> String {
        let event_type = match self {
            AccountEvent::AccountRegistered { .. } => "AccountRegistered",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
