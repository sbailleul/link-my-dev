use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub enum AccountCommand {
    Register { account_id: String, email: String },
}
