use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum TeamCommand{
    Create{ team_id: String, name: String}
}
