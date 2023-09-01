use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum TeamCommand {
    Create { team_id: Uuid, name: String, owner_id: Uuid },
    ChangeName(String),
}
