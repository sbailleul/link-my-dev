
use std::sync::Arc;
use std::task::Poll;
use std::thread::Builder;


use actix_web::{web, Responder, HttpResponse};
use anyhow::Result;
use cqrs_es::persist::PersistedEventStore;
use cqrs_es::EventStore;
use postgres_es::PostgresEventRepository;
use serde::{Deserialize, Serialize};


use crate::collaboration::bootstrap::CollaborationState;
use crate::collaboration::domain::commands::TeamCommand;
use crate::collaboration::domain::team::Team;

use crate::Config;

use super::bootstrap::PostGresCollaborationState;

#[derive(Deserialize, Debug, Serialize, Clone)]
struct CreateTeamRequest {
    team_id: String,
    name: String,
}
async fn create_team<ES: EventStore<Team>, C: CollaborationState<ES>>(
    web::Json(request): web::Json<CreateTeamRequest>,
    data: web::Data<C>,
) -> impl Responder {
    let cmd = request.clone();
    data.cqrs().execute("", TeamCommand::Create { team_id: request.team_id, name: request.name }).await;
    HttpResponse::Ok().json(cmd)
}

pub  fn config_web_collaboration(app_config: &mut web::ServiceConfig)  {
    let create_team = create_team::<PersistedEventStore<PostgresEventRepository, Team>,PostGresCollaborationState >;
    app_config.service(
        web::scope("/teams")
            .route("", web::post().to(create_team)),
    );
}

pub async fn  provide_collaboration_state(config: &Config)->Result<PostGresCollaborationState>{
    PostGresCollaborationState::new(config).await
}