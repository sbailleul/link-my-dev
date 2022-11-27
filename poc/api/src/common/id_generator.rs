use uuid::Uuid;

pub trait IdGenerator{
    fn new_id(&self) -> Uuid;
}

pub struct UuidGenerator;

impl IdGenerator for UuidGenerator {
    fn new_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}