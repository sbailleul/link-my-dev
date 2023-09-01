use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct AccountView {
    pub id: String,
    pub name: String,
}
