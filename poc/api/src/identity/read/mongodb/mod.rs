pub mod accounts;







use crate::identity::{
    config::IdentityConfig,
};

use anyhow::Result;





use mongodb::{options::ClientOptions, Client};





pub async fn get_client(config: &IdentityConfig) -> Result<Client> {
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
