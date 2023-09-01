pub mod accounts;

use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    web::{self, Data},
    Error,
};
use cqrs_es::persist::PersistedEventStore;
use mongodb::Client;
use postgres_es::PostgresEventRepository;
use sqlx::{Pool, Postgres};

use crate::{common::web::Bootstrap, identity::domain::account::Account};

use self::accounts::{create_account, get_account, get_accounts};

use super::{
    bootstrap::PostGresIdentityState, config::IdentityConfig, persistence::postgres::get_pool,
    read::mongodb::get_client,
};
use anyhow::Result;
#[derive(Clone)]
pub struct IdentityWeb {
    config: IdentityConfig,
    postgres_pool: Pool<Postgres>,
    mongodb_client: Client,
}

impl IdentityWeb {
    pub async fn new(config: IdentityConfig) -> Result<Self> {
        let postgres_pool = get_pool(&config).await?;
        let mongodb_client = get_client(&config).await?;
        Ok(Self {
            config,
            postgres_pool,
            mongodb_client,
        })
    }
}
impl Bootstrap for IdentityWeb {
    fn bootstrap<T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>>(
        &self,
        app: actix_web::App<T>,
    ) -> actix_web::App<T> {
        app.configure(|service_conf| {
            service_conf
                .app_data(Data::new(
                    PostGresIdentityState::new(
                        self.postgres_pool.clone(),
                        self.mongodb_client.clone(),
                        &self.config,
                    )
                    .unwrap(),
                ))
                .service(
                    web::scope("/accounts")
                        .route(
                            "",
                            web::post().to(create_account::<
                                PersistedEventStore<PostgresEventRepository, Account>,
                                PostGresIdentityState,
                            >),
                        )
                        .route(
                            "",
                            web::get().to(get_accounts::<
                                PersistedEventStore<PostgresEventRepository, Account>,
                                PostGresIdentityState,
                            >),
                        )
                        .service(web::scope("/{account_id}").route(
                            "",
                            web::get().to(get_account::<
                                PersistedEventStore<PostgresEventRepository, Account>,
                                PostGresIdentityState,
                            >),
                        )),
                );
        })
    }
}
