use std::env;

use anyhow::Result;
use dotenv_codegen::dotenv;
use postgres_es::default_postgress_pool;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive( EnumString, Debug, Clone)]
pub enum Environment {
    DEVELOP,
}


#[derive(Debug, Clone)]
pub struct Config {
    
    pub rust_log: Option<String>,
    pub rust_backtrace: Option<String>,
    pub env: Environment,
    pub collaboration_connection_string: Option<String>,
}

impl Config {
    pub  fn new() -> Result<Self> {
        let env = Environment::from_str(dotenv!("ENVIRONMENT"));
        Ok(Self {
            env: env.unwrap_or(Environment::DEVELOP),
            collaboration_connection_string: Some(dotenv!("DATABASE_URL").to_owned()),
            rust_log: Some(dotenv!("RUST_LOG").to_owned()),
            rust_backtrace: Some(dotenv!("RUST_BACKTRACE").to_owned()),
        })
    }
}


