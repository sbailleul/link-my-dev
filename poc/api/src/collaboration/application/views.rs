use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamView {
    pub id: Uuid,
    pub name: String,
}
