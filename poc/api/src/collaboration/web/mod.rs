use std::sync::Arc;
use std::task::Poll;
use std::thread::Builder;

use anyhow::Result;
use cqrs_es::{CqrsFramework, Query};
use postgres_es::{PostgresCqrs, PostgresEventRepository, PostgresViewRepository};
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::collaboration::bootstrap::CollaborationState;
use crate::collaboration::domain::commands::TeamCommand;
use crate::collaboration::domain::team::Team;
use crate::collaboration::store::postgres::configure;
use crate::Config;

#[derive(Deserialize, Debug)]
struct CreateTeamRequest{team_id: String, name: String}
#[post("/teams", data = "<cmd>")]
fn create_team(cmd: Json<CreateTeamRequest>, state: &State<CollaborationState>) -> &str {
    dbg!(&cmd);
    state.cqrs.execute("", TeamCommand::Create { team_id: cmd.0.team_id, name: cmd.0.name });
    "CQRS"
}


pub async fn add_collaboration(builder:   Rocket<Build>, config: &Config) -> Result<Rocket<Build>>{
    let collaboration_state = CollaborationState::new(config).await?;
    Ok(builder.manage(collaboration_state).mount("/", routes![create_team]))
}

