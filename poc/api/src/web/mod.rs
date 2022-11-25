use actix_web::{web::Data, App, HttpServer};
use anyhow::Result;

use crate::{
    collaboration::{persistence::postgres::get_pool, web::config_web_collaboration, read::mongodb::get_client},
    common::web::AppData,
    Config,
};

pub async fn launch_actix(config: &'static Config) -> Result<()> {
    let collaboration_postgres_pool = get_pool(&config.collaboration_config).await?;
    let collaboration_mongodb_client = get_client(&config.collaboration_config).await?;
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppData::new()))
            .configure(|service_conf| {
                config_web_collaboration(service_conf, collaboration_postgres_pool.clone())
            })
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?)
}
