use actix_web::{web::Data, App, HttpServer};
use anyhow::Result;

use crate::{
    collaboration::{web::CollaborationWeb},
    common::web::{AppData, Bootstrap},
    identity::{web::IdentityWeb},
    Config,
};

pub async fn launch_actix(config: &'static Config) -> Result<()> {
    let collaboration_web = CollaborationWeb::new(config.collaboration_config.clone()).await?;
    let identity_web = IdentityWeb::new(config.identity_config.clone()).await?;
    Ok(HttpServer::new(move || {
        let mut app = App::new();
        app = app.app_data(Data::new(AppData::new()));
        app = collaboration_web.bootstrap(app);
        app = identity_web.bootstrap(app);
        app
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?)
}
