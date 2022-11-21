use anyhow::Result;

use rocket::{Ignite, Rocket};

use crate::collaboration::web::add_collaboration;
use crate::Config;

pub async fn launch_rocket(config: &Config) -> Result<Rocket<Ignite>> {
    let builder = rocket::build();
    let builder = add_collaboration( builder, &config).await?;
    Ok(builder.launch().await?)
}