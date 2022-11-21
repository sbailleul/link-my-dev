use std::env;

use anyhow::Result;
use dotenv_codegen::dotenv;
use postgres_es::default_postgress_pool;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive( EnumString)]
pub enum Environment {
    DEVELOP,
}



pub struct Config {
    pub env: Environment,
    pub collaboration_connection_string: Option<String>,
}

impl Config {
    pub async fn new() -> Result<Self> {
        let env = Environment::from_str(dotenv!("ENVIRONMENT"));
        Ok(Self {
            env: env.unwrap_or(Environment::DEVELOP),
            collaboration_connection_string: Some(dotenv!("DATABASE_URL").to_owned()),
        })
    }
}
