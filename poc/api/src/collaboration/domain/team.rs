use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::collaboration::domain::commands::TeamCommand;
use crate::collaboration::domain::events::TeamEvent;
use crate::collaboration::domain::events::TeamEvent::TeamCreated;
use crate::common::domain::entity::Entity;
use thiserror::Error;

use super::member::{MemberId, Member};

#[derive(Error, Debug)]
pub enum TeamError{
    #[error("Invalid name {0}")]
    InvalidName(String)
}
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct TeamId(Uuid);

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct Team{
    name: String,
    id: TeamId,
    owner: Member
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
            TeamCommand::Create { team_id, name , owner_id: Uuid} => {
                TeamCreated {id: team_id, name}
            }
            TeamCommand::ChangeName(name) => TeamEvent::NameChanged { id: self.id.0, name },
        };
        Ok(vec![event])
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            TeamCreated { name, id    } => {
                self.name = name;
                self.id = TeamId(id);
            }
            TeamEvent::NameChanged { id: _, name } => self.name = name
        }
    }
}

