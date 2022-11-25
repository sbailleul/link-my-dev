#[derive(Debug,Clone)]
pub struct CollaborationConfig{
    pub event_store_connection_string: Option<String>,
    pub read_store_connection_string: Option<String>,
    pub read_store_database: Option<String>,
    pub read_store_teams_collection: Option<String>,
    pub read_store_app_name: Option<String>
}

impl CollaborationConfig{
    pub fn new()-> Self{
        Self { 
            event_store_connection_string: Some(dotenv!("POSTGRES_COLLABORATION_CONNECTION_STRING").to_owned()),
            read_store_connection_string: Some(dotenv!("MONGO_COLLABORATION_CONNECTION_STRING").to_owned()),
            read_store_database: Some(dotenv!("MONGO_COLLABORATION_DATABASE").to_owned()),
            read_store_teams_collection: Some(dotenv!("MONGO_COLLABORATION_TEAMS_COLLECTION").to_owned()),
            read_store_app_name: Some(dotenv!("MONGO_COLLABORATION_APP_NAME").to_owned()),
         }
    }
}