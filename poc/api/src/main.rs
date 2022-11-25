#![feature(fn_traits)]

#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate async_trait;
extern crate env_logger;


use anyhow::Result;


use web::launch_actix;

use crate::config::Config;

mod collaboration;
mod config;
mod web;
mod common;

lazy_static::lazy_static! {
    static ref CONFIG: Config = Config::new().unwrap();
}

#[actix_web::main]
async fn main() -> Result<()> {
    if let Some(rust_log) = CONFIG.clone().rust_log {
        std::env::set_var("RUST_LOG", rust_log);
    }
    if let Some(rust_backtrace) = CONFIG.clone().rust_backtrace {
        std::env::set_var("RUST_BACKTRACE", rust_backtrace);
    }
    env_logger::init();
    dbg!(&CONFIG.clone());
    launch_actix(&CONFIG).await?;
    Ok(())
}
