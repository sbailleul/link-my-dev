use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use cqrs_es::EventStore;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::web::AppData,
    identity::{
        application::{AccountViewRepository, IdentityState},
        domain::{account::Account, commands::AccountCommand},
    },
};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CreateAccountRequest {
    id: String,
    email: String,
}

pub async fn create_account<ES: EventStore<Account>, C: IdentityState<ES>>(
    web::Json(request): web::Json<CreateAccountRequest>,
    identity: web::Data<C>,
    _app_data: web::Data<AppData>,
) -> impl Responder {
    let cmd = request.clone();
    identity
        .cqrs()
        .execute(
            &request.id,
            AccountCommand::Register {
                account_id: request.id.clone(),
                email: request.email.clone(),
            },
        )
        .await;
    HttpResponse::Ok().json(cmd)
}

pub async fn get_accounts<ES: EventStore<Account>, C: IdentityState<ES>>(
    identity: web::Data<C>,
) -> impl Responder {
    HttpResponse::Ok().json(identity.account_views().get_all().await.unwrap())
}

pub async fn get_account<ES: EventStore<Account>, C: IdentityState<ES>>(
    account_id: web::Path<Uuid>,
    identity: web::Data<C>,
) -> impl Responder {
    HttpResponse::Ok().json(
        identity
            .account_views()
            .get_by_id(&account_id)
            .await
            .unwrap(),
    )
}
