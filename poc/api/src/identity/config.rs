#[derive(Debug, Clone)]
pub struct IdentityConfig {
    pub event_store_connection_string: Option<String>,
    pub read_store_connection_string: Option<String>,
    pub read_store_database: Option<String>,
    pub read_store_accounts_collection: Option<String>,
    pub read_store_app_name: Option<String>,
}

impl IdentityConfig {
    pub fn new() -> Self {
        Self {
            event_store_connection_string: Some(
                dotenv!("POSTGRES_IDENTITY_CONNECTION_STRING").to_owned(),
            ),
            read_store_connection_string: Some(
                dotenv!("MONGO_IDENTITY_CONNECTION_STRING").to_owned(),
            ),
            read_store_database: Some(dotenv!("MONGO_IDENTITY_DATABASE").to_owned()),
            read_store_accounts_collection: Some(
                dotenv!("MONGO_IDENTITY_ACCOUNTS_COLLECTION").to_owned(),
            ),
            read_store_app_name: Some(dotenv!("MONGO_IDENTITY_APP_NAME").to_owned()),
        }
    }
}
