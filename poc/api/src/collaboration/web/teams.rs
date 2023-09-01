use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use cqrs_es::EventStore;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    collaboration::{
        application::{CollaborationState, TeamViewRepository},
        domain::{commands::TeamCommand, team::Team},
    },
    common::web::AppData,
};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CreateTeamRequest {
    name: String,
    ownerId: Uuid
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
            &team_id.to_string(),
            TeamCommand::Create {
                team_id,
                name: request.name,
                owner_id: request.ownerId
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
    team_id: web::Path<Uuid>,
    web::Json(request): web::Json<UpdateTeamRequest>,
    collaboration: web::Data<C>,
) -> impl Responder {
    let cmd = request.clone();
    let team_id = team_id.into_inner();
    collaboration
        .cqrs()
        .execute(&team_id.to_string(), TeamCommand::ChangeName(request.name))
        .await;
    HttpResponse::Ok().json(cmd)
}

pub async fn get_teams<ES: EventStore<Team>, C: CollaborationState<ES>>(
    collaboration: web::Data<C>,
) -> impl Responder {
    HttpResponse::Ok().json(collaboration.team_views().get_all().await.unwrap())
}

pub async fn get_team<ES: EventStore<Team>, C: CollaborationState<ES>>(
    team_id: web::Path<Uuid>,
    collaboration: web::Data<C>,
) -> impl Responder {
    HttpResponse::Ok().json(
        collaboration
            .team_views()
            .get_by_id(&team_id)
            .await
            .unwrap(),
    )
}
