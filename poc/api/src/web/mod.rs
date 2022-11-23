use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use anyhow::Result;

use crate::{
    collaboration::web::{config_web_collaboration, provide_collaboration_state},
    Config,
};

pub async fn launch_actix(config: &'static Config) -> Result<()> {
    Ok(HttpServer::new(|| {
        App::new()
            .data_factory(|| async {
                 provide_collaboration_state(config).await
            })
            .configure(config_web_collaboration)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?)
}
