use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};


use crate::identity::domain::commands::AccountCommand;
use crate::identity::domain::events::AccountEvent;
use crate::identity::domain::events::AccountEvent::AccountRegistered;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("Invalid name {0}")]
    InvalidName(String),
}

#[derive(Serialize, Deserialize, Default)]
pub struct AccountId(String);

#[derive(Serialize, Default, Deserialize)]
pub struct Account {
    email: String,
    id: AccountId,
}

#[async_trait]
impl Aggregate for Account {
    type Command = AccountCommand;
    type Event = AccountEvent;
    type Error = AccountError;
    type Services = ();

    fn aggregate_type() -> String {
        "Account".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        dbg!(&command);
        let event = match command {
            AccountCommand::Register { account_id, email } => AccountRegistered {
                id: account_id,
                email,
            },
        };
        Ok(vec![event])
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            AccountRegistered { email, id } => {
                self.email = email;
                self.id = AccountId(id);
            }
        }
    }
}
