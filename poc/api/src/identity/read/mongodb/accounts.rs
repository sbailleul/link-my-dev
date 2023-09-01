use crate::identity::domain::events::AccountEvent;
use crate::{
    common::mongodb::client::MongoClient,
    identity::{
        application::{self, views::AccountView},
        domain::account::Account,
    },
};
use anyhow::Result;
use bson::{doc};
use cqrs_es::{EventEnvelope, Query};
use futures::stream::{StreamExt, TryStreamExt};
use mapper::Mapper;
use mongodb::Client;
use serde::{Deserialize, Serialize};


#[derive(Mapper, Deserialize, Serialize, Debug)]
#[to(AccountView, strategy=into)]
struct AccountDocument {
    #[to(AccountView, field=id)]
    #[serde(with = "bson::serde_helpers::hex_string_as_object_id")]
    _id: String,
    name: String,
}

#[derive(Clone)]
pub struct AccountViewRepository {
    client: MongoClient,
}
impl AccountViewRepository {
    pub fn new(client: Client, database: String, collection: String) -> Self {
        Self {
            client: MongoClient::new(client, database, collection),
        }
    }
}
#[async_trait]
impl application::AccountViewRepository for AccountViewRepository {
    async fn get_all(&self) -> Result<Vec<AccountView>> {
        let collection = self.client.collection::<AccountDocument>();
        let items = collection.await.find(doc! {}, None).await?;
        Ok(items
            .map(|doc| doc.map(|d: AccountDocument| d.into()))
            .try_collect::<Vec<AccountView>>()
            .await?)
    }
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Option<AccountView>> {
        let collection = self.client.collection::<AccountDocument>();
        let account_doc = collection.await.find_one(doc! {"_id": id}, None).await?;
        Ok(account_doc.map(|doc: AccountDocument| AccountView {
            id: doc._id,
            name: doc.name,
        }))
    }
}

#[async_trait]
impl Query<Account> for AccountViewRepository {
    async fn dispatch(&self, _aggregate_id: &str, events: &[EventEnvelope<Account>]) {
        for event in events {
            match &event.payload {
                AccountEvent::AccountRegistered { id, email: name } => {
                    self.client
                        .collection::<AccountDocument>()
                        .await
                        .insert_one(
                            &AccountDocument {
                                _id: id.clone(),
                                name: name.clone(),
                            },
                            None,
                        )
                        .await;
                }
            }
        }
    }
}
