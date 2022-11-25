use cqrs_es::{Aggregate};
use serde::{Deserialize, Serialize};

use crate::collaboration::domain::commands::TeamCommand;
use crate::collaboration::domain::events::TeamEvent;
use crate::collaboration::domain::events::TeamEvent::TeamCreated;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TeamError{
    #[error("Invalid name {0}")]
    InvalidName(String)
}

#[derive(Serialize, Default, Deserialize)]
pub struct Team{
    name: String,
    id: String
}

#[async_trait]
impl Aggregate for Team{
    type Command = TeamCommand;
    type Event = TeamEvent;
    type Error = TeamError;
    type Services = ();

    fn aggregate_type() -> String {
        "Team".to_string()
    }

    async fn handle(&self, command: Self::Command, _service: &Self::Services) -> Result<Vec<Self::Event>, Self::Error> {
        dbg!(&command);
        let event = match command {
            TeamCommand::Create { team_id, name } => {
                TeamCreated {id: team_id, name}
            }
            TeamCommand::ChangeName(name) => TeamEvent::NameChanged { id: self.id.clone(), name },
        };
        Ok(vec![event])
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            TeamCreated { name, id    } => {
                self.name = name;
                self.id = id;
            }
            TeamEvent::NameChanged { id: _, name } => self.name = name
        }
    }
}