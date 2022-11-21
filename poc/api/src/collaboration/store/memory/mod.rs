use cqrs_es::CqrsFramework;
use cqrs_es::mem_store::MemStore;

// use crate::collaboration::domain::commands::TeamCommand;
// use crate::collaboration::domain::team::Team;
// use crate::collaboration::store::postgres::configure_repo;

// #[tokio::test]
// async fn test_event_store() {
//     let event_store = MemStore::<Team>::default();
//     let query = SimpleLoggingQuery{};
//     let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)], ());
//
//     let aggregate_id = "aggregate-instance-A";
//
//     // deposit $1000
//     cqrs.execute(aggregate_id, TeamCommand::Create {
//         name: "ABC".to_string(),
//         team_id: "TEST".to_string()
//     }).unwrap();
// }
