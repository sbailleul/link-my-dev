use std::sync::Arc;

use super::id_generator::{IdGenerator, UuidGenerator};

pub struct AppData{
    pub id_generator: Arc<dyn IdGenerator>
}

impl AppData {
    pub fn new()-> Self{
        Self { id_generator: Arc::new(UuidGenerator) }
    }
}