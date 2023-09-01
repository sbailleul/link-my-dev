use std::sync::Arc;

use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    App, Error,
};

use super::id_generator::{IdGenerator, UuidGenerator};

pub struct AppData {
    pub id_generator: Arc<dyn IdGenerator>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            id_generator: Arc::new(UuidGenerator),
        }
    }
}

pub trait Bootstrap: Clone {
    fn bootstrap<T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>>(
        &self,
        app: App<T>,
    ) -> App<T>;
}
