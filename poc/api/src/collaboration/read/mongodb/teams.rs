use crate::collaboration::domain::events::TeamEvent;
use crate::{
    collaboration::{
        application::{self, views::TeamView},
        domain::team::Team,
    },
    common::mongodb::client::MongoClient,
};
use anyhow::Result;
use bson::{doc, to_document};
use cqrs_es::{EventEnvelope, Query};
use futures::stream::{StreamExt, TryStreamExt};
use mapper::Mapper;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Mapper, Deserialize, Serialize, Debug)]
#[to(TeamView, strategy=into)]
struct TeamDocument {
    #[to(TeamView, field=id)]
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    _id: Uuid,
    name: String,
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
        let collection = self.client.collection::<TeamDocument>();
        let items = collection.await.find(doc! {}, None).await?;
        Ok(items
            .map(|doc| doc.map(|d: TeamDocument| d.into()))
            .try_collect::<Vec<TeamView>>()
            .await?)
    }
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Option<TeamView>> {
        let collection = self.client.collection::<TeamDocument>();
        let team_doc = collection.await.find_one(doc! {"_id": id}, None).await?;
        Ok(team_doc.map(|doc: TeamDocument| TeamView {
            id: doc._id,
            name: doc.name,
        }))
    }
}

#[async_trait]
impl Query<Team> for TeamViewRepository {
    async fn dispatch(&self, _aggregate_id: &str, events: &[EventEnvelope<Team>]) {
        println!("EVENTS");
        dbg!(events);
        for event in events {
            match &event.payload {
                TeamEvent::TeamCreated { id, name } => {
                    let result = self.client
                        .collection::<TeamDocument>()
                        .await
                        .insert_one(
                            &TeamDocument {
                                _id: *id,
                                name: name.clone(),
                            },
                            None,
                        )
                        .await.expect("Insert team failed");
                    dbg!("New doc:{}", result.inserted_id);
                }
                TeamEvent::NameChanged { id, name } => {
                    self.client.collection::<TeamDocument>()
                        .await
                        .find_one_and_update(doc! {"_id": *id}, doc! {"$set": {"name": name}}, None)
                        .await.expect("Error");
                }
            }
        }
    }
}
