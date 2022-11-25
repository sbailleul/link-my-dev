use mongodb::{options::ClientOptions, Client};
use anyhow::Result;
use crate::collaboration::config::CollaborationConfig;

pub async fn get_client(config: &CollaborationConfig) -> Result<Client> {
    let mut client_options =
        ClientOptions::parse(config.read_store_connection_string.clone().unwrap().as_str()).await?;
    client_options.app_name = Some("Housing Scraper".to_string());
    Ok(Client::with_options(client_options)?)
}
