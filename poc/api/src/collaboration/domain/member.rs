use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::domain::entity::Entity;

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct MemberId(Uuid);
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct Member{
    id: MemberId,
}

impl Entity<MemberId> for Member{
    fn id(&self) -> &MemberId {
        &self.id
    }
}



