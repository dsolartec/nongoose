use mongodb::{
  bson::{doc, Bson, Document},
  Database,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::Result;

/// Schema
///
/// This trait is defined through the [`async-trait`](https://crates.io/crates/async-trait) macro.
#[async_trait::async_trait]
pub trait Schema: DeserializeOwned + Serialize {
  type __SchemaId: Into<Bson> + Clone + Send;

  fn __get_collection_name() -> String;

  fn __get_id(&self) -> Self::__SchemaId;

  fn __get_id_query(&self) -> Document {
    doc! { "_id": self.__get_id().into() }
  }

  fn __to_document(&self) -> Result<Document>;

  async fn __check_unique_fields(&self, database: &Database) -> Result<()>;

  fn __get_relations() -> Option<Vec<Document>>;
}
