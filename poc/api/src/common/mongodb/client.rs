
use mongodb::{Client};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct MongoClient {
    client: Client,
    database: String,
    collection: String,
}
impl MongoClient {
    pub fn new(client: Client, database: String, collection: String) -> Self {
        Self {
            client,
            collection,
            database,
        }
    }
    pub async fn collection<'de, Item: Serialize + Deserialize<'de>>(
        &self,
    ) -> mongodb::Collection<Item> {
        
        self
            .client
            .database(&self.database)
            .collection::<Item>(&self.collection)
    }
}
