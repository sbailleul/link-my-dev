pub mod teams;

use actix_web::web::{self, Data};
use cqrs_es::persist::PersistedEventStore;
use mongodb::Client;
use postgres_es::PostgresEventRepository;
use sqlx::{Pool, Postgres};

use crate::collaboration::domain::team::Team;

use self::teams::{create_team, update_team, get_teams};

use super::{bootstrap::PostGresCollaborationState, config::CollaborationConfig};

pub fn config_web_collaboration(app_config: &mut web::ServiceConfig, db_pool: Pool<Postgres>, client: Client, config: &CollaborationConfig) {
    app_config
        .app_data(Data::new(PostGresCollaborationState::new(db_pool, client, config).unwrap()))
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
                .service(web::scope("/{team_id}").route(
                    "",
                    web::put().to(update_team::<
                        PersistedEventStore<PostgresEventRepository, Team>,
                        PostGresCollaborationState,
                    >),
                )),
        );
}
