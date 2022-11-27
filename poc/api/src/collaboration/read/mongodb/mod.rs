use crate::collaboration::domain::events::TeamEvent;
use crate::collaboration::{
    application::{self, views::TeamView},
    config::CollaborationConfig,
    domain::team::Team,
};
use crate::common::mongodb::client::MongoClient;
use crate::common::mongodb::item::to_document_item;
use anyhow::Result;

use cqrs_es::{EventEnvelope, Query};
use futures::TryStreamExt;
use mongodb::bson::doc;
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

#[derive(Clone)]
pub struct TeamViewRepository {
    client: MongoClient,
}
impl TeamViewRepository {
    pub fn new(client: Client, database: String, collection: String) -> Self {
        Self {
            client: MongoClient::new(client, database, collection),
        }
    }
}
#[async_trait]
impl application::TeamViewRepository for TeamViewRepository {
    async fn get_all(&self) -> Result<Vec<TeamView>> {
        let collection = self.client.collection::<TeamView>();
        let items = collection.find(doc! {}, None).await?;
        Ok(items.try_collect::<Vec<TeamView>>().await?)
    }
}

#[async_trait]
impl Query<Team> for TeamViewRepository {
    async fn dispatch(&self, _aggregate_id: &str, events: &[EventEnvelope<Team>]) {
        for event in events {
            match &event.payload {
                TeamEvent::TeamCreated { id, name } => {
                    let doc = to_document_item(&TeamView{id: id.clone(), name: name.clone()}).unwrap();
                    self.client.collection().insert_one(
                        doc,
                        None
                    ).await;
                }
                TeamEvent::NameChanged { id, name } => {
                    self.client.collection::<TeamView>().find_one_and_update(
                        doc! {"_id": id},
                        doc! {"name": name},
                        None,
                    ).await;
                }
            }
        }
    }
}
