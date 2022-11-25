use uuid::Uuid;

pub trait IdGenerator{
    fn new_id(&self) -> String;
}

pub struct UuidGenerator;

impl IdGenerator for UuidGenerator {
    fn new_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}