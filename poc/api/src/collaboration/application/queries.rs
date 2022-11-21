use cqrs_es::{EventEnvelope, Query, View};
use crate::collaboration::domain::team::{Team, TeamEvent};

pub struct SimpleLoggingQuery {}

#[async_trait]
impl Query<Team> for SimpleLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Team>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
        }
    }
}

// Our second query, this one will be handled with Postgres `GenericQuery`
// which will serialize and persist our view after it is updated. It also
// provides a `load` method to deserialize the view on request.
pub type TeamQuery = GenericQuery<
    PostgresViewRepository<TeamView, BankAccount>,
    BankAccountView,
    BankAccount,
>;

// The view for a BankAccount query, for a standard http application this should
// be designed to reflect the response dto that will be returned to a user.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TeamView {
    id: String,
    name: f64,
}



// This updates the view with events as they are committed.
// The logic should be minimal here, e.g., don't calculate the account balance,
// design the events to carry the balance information instead.
impl View<Team> for TeamView {
    fn update(&mut self, event: &EventEnvelope<Team>) {
        match &event.payload {
            TeamEvent::TeamCreated { name } => {
                self.name
            }
        }
    }
}