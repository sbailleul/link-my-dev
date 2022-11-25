use actix_web::{web, HttpResponse, Responder};
use cqrs_es::EventStore;
use serde::{Deserialize, Serialize};

use crate::{
    collaboration::{
        domain::{commands::TeamCommand, team::Team}, application::CollaborationState,
    },
    common::web::AppData,
};



#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CreateTeamRequest {
    name: String,
}

pub async fn create_team<ES: EventStore<Team>, C: CollaborationState<ES>>(
    web::Json(request): web::Json<CreateTeamRequest>,
    collaboration: web::Data<C>,
    app_data: web::Data<AppData>,
) -> impl Responder {
    let cmd = request.clone();
    let team_id = app_data.id_generator.new_id();
    collaboration
        .cqrs()
        .execute(
            &team_id.clone(),
            TeamCommand::Create {
                team_id,
                name: request.name,
            },
        )
        .await;
    HttpResponse::Ok().json(cmd)
}


#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct UpdateTeamRequest {
    name: String,
}
pub async fn update_team<ES: EventStore<Team>, C: CollaborationState<ES>>(
    team_id: web::Path<String>,
    web::Json(request): web::Json<UpdateTeamRequest>,
    collaboration: web::Data<C>,
) -> impl Responder {
    let cmd = request.clone();
    let team_id = team_id.into_inner();
    collaboration
        .cqrs()
        .execute(
            &team_id,
            TeamCommand::Create {
                team_id: team_id.clone(),
                name: request.name,
            },
        )
        .await;
    HttpResponse::Ok().json(cmd)
}

pub async fn get_teams<ES: EventStore<Team>, C: CollaborationState<ES>>(
    collaboration: web::Data<C>,
) -> impl Responder {
    HttpResponse::Ok()
}
