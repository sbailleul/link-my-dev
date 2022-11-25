

use anyhow::Result;
use dotenv_codegen::dotenv;



use std::str::FromStr;
use strum_macros::EnumString;

use crate::collaboration::config::CollaborationConfig;

#[derive( EnumString, Debug, Clone)]
pub enum Environment {
    DEVELOP,
}


#[derive(Debug, Clone)]
pub struct Config {
    pub rust_log: Option<String>,
    pub rust_backtrace: Option<String>,
    pub env: Environment,
    pub collaboration_config: CollaborationConfig
}

impl Config {
    pub  fn new() -> Result<Self> {
        let env = Environment::from_str(dotenv!("ENVIRONMENT"));
        Ok(Self {
            env: env.unwrap_or(Environment::DEVELOP),
            rust_log: Some(dotenv!("RUST_LOG").to_owned()),
            rust_backtrace: Some(dotenv!("RUST_BACKTRACE").to_owned()),
            collaboration_config: CollaborationConfig::new()
        })
    }
}


