pub mod types;

use std::sync::Mutex;

use mongodb::{
  bson::{doc, to_bson, Bson, Document},
  sync::Database,
};
use serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "async")]
use tokio::task::spawn_blocking;

use crate::{errors::Result, NongooseBuilder};

/// Schema
///
/// This trait is defined through the [`async-trait`](https://crates.io/crates/async-trait) macro.
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait Schema: DeserializeOwned + Serialize + Send {
  type __SchemaId: Into<Bson> + Clone + Send;

  fn __get_instance(instance: Option<NongooseBuilder>) -> &'static Mutex<NongooseBuilder>;

  fn __get_collection_name() -> String;

  fn __get_id(&self) -> Self::__SchemaId;

  fn __get_id_query(&self) -> Document {
    doc! { "_id": self.__get_id().into() }
  }

  fn __to_document(&self) -> Result<Document> {
    match to_bson(self)? {
      Bson::Document(document) => Ok(document),
      _ => unreachable!(),
    }
  }

  fn __check_unique_fields(&self, database: &Database) -> Result<()>;

  fn __get_relations(&self) -> Option<Vec<types::SchemaRelation>>;

  fn __set_relations(&mut self, field: &str, new_value: Bson) -> Result<()>;

  fn __populate_sync(mut self, field: &str) -> Result<Self> {
    if let Some(relations) = self.__get_relations() {
      for relation in relations.iter() {
        if relation.field_ident == field {
          if let Some(data) = Self::__get_instance(None)
            .lock()
            .unwrap()
            .database
            .collection::<Document>(relation.schema_name.as_str())
            .find_one(Some(doc! { "_id": relation.field_value.clone() }), None)?
          {
            self.__set_relations(field, Bson::Document(data))?;
          }
        }
      }
    }

    Ok(self)
  }

  #[cfg(not(feature = "async"))]
  fn populate(self, field: &str) -> Result<Self> {
    self.__populate_sync(field)
  }

  #[cfg(feature = "async")]
  async fn populate(mut self, field: &'static str) -> Result<Self>
  where
    Self: 'static,
  {
    spawn_blocking(move || self.__populate_sync(field)).await?
  }
}
