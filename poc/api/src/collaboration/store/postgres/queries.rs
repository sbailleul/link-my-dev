use cqrs_es::persist::{GenericQuery, ViewRepository};
use postgres_es::PostgresViewRepository;
use crate::collaboration::application::queries::TeamView;
use crate::collaboration::domain::team::Team;

pub type TeamQuery = GenericQuery<
    PostgresViewRepository<TeamView, Team>,
    TeamView,
    Team,
>;
