pub mod teams;

use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    web::{self, Data},
    Error,
};
use cqrs_es::persist::PersistedEventStore;
use mongodb::Client;
use postgres_es::PostgresEventRepository;
use sqlx::{Pool, Postgres};

use crate::{collaboration::domain::team::Team, common::web::Bootstrap};

use self::teams::{create_team, get_team, get_teams, update_team};

use super::{
    bootstrap::PostGresCollaborationState, config::CollaborationConfig,
    persistence::postgres::get_pool, read::mongodb::get_client,
};
use anyhow::Result;

#[derive(Clone)]
pub struct CollaborationWeb {
    config: CollaborationConfig,
    postgres_pool: Pool<Postgres>,
    mongodb_client: Client,
}

impl CollaborationWeb {
    pub async fn new(config: CollaborationConfig) -> Result<Self> {
        let postgres_pool = get_pool(&config).await?;
        let mongodb_client = get_client(&config).await?;
        Ok(Self {
            config,
            postgres_pool,
            mongodb_client,
        })
    }
}

impl Bootstrap for CollaborationWeb {
    fn bootstrap<T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>>(
        &self,
        app: actix_web::App<T>,
    ) -> actix_web::App<T> {
        app.configure(|service_conf| {
            service_conf
                .app_data(Data::new(
                    PostGresCollaborationState::new(
                        self.postgres_pool.clone(),
                        self.mongodb_client.clone(),
                        &self.config,
                    )
                    .unwrap(),
                ))
                .service(
                    web::scope("/teams")
                        .route(
                            "",
                            web::post().to(create_team::<
                                PersistedEventStore<PostgresEventRepository, Team>,
                                PostGresCollaborationState,
                            >),
                        )
                        .route(
                            "",
                            web::get().to(get_teams::<
                                PersistedEventStore<PostgresEventRepository, Team>,
                                PostGresCollaborationState,
                            >),
                        )
                        .service(
                            web::scope("/{team_id}")
                                .route(
                                    "",
                                    web::put().to(update_team::<
                                        PersistedEventStore<PostgresEventRepository, Team>,
                                        PostGresCollaborationState,
                                    >),
                                )
                                .route(
                                    "",
                                    web::get().to(get_team::<
                                        PersistedEventStore<PostgresEventRepository, Team>,
                                        PostGresCollaborationState,
                                    >),
                                ),
                        ),
                );
        })
    }
}
