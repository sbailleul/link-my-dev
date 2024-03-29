pub mod teams;





use crate::collaboration::{
    config::CollaborationConfig,
};



use anyhow::Result;





use mongodb::{options::ClientOptions, Client};





pub async fn get_client(config: &CollaborationConfig) -> Result<Client> {
    let mut client_options = ClientOptions::parse(
        config
            .read_store_connection_string
            .clone()
            .unwrap()
            .as_str(),
    )
    .await?;
    client_options.app_name = config.read_store_app_name.clone();
    Ok(Client::with_options(client_options)?)
}
