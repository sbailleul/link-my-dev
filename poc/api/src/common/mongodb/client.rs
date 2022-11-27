

use mongodb::{Client};
use serde::Serialize;



#[derive(Clone)]
pub struct MongoClient{
    client: Client,
    database: String,
    collection: String
}
impl MongoClient {
    pub fn new(client: Client, database: String, collection: String)-> Self{
        Self{client,collection,database}
    }
    pub fn collection<Item: Serialize>(&self) -> mongodb::Collection<Item>{
        self.client.database(&self.database).collection::<Item>(&self.collection)
    }
}
