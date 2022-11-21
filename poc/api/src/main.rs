#[macro_use] extern crate dotenv_codegen;
#[macro_use] extern crate rocket;
#[macro_use] extern crate strum_macros;

use std::env;

use anyhow::Result;
use dotenv::dotenv;
use postgres_es::default_postgress_pool;

use crate::bootstrap::Config;
use crate::web::launch_rocket;

mod collaboration;
mod web;
mod bootstrap;

#[post("/")]
fn index() -> &'static str {
    "Hello, world!"
}



#[rocket::main]
async fn main() -> Result<()> {
    let bootstrap = Config::new().await?;
    launch_rocket(&bootstrap).await?;
    Ok(())
}