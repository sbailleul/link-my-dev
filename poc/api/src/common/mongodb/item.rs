use bson::{ser::Result, to_document, Document};
use serde::Serialize;


pub fn to_document_item<Item: Serialize>(item: &Item) -> Result<Document> {
    let mut doc = to_document(item)?;
    if let Some(id) = doc.remove("id") {
        doc.insert("_id", id);
    }
    Ok(doc)
}
