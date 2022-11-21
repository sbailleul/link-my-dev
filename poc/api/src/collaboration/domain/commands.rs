use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum TeamCommand{
    Create{ team_id: String, name: String}
}
