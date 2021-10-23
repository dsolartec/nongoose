mod data;
pub mod types;

pub use data::SchemaData;
use mongodb::{
  bson::{bson, doc, Bson, Document},
  sync::Database,
};
use serde::{de::DeserializeOwned, Serialize};
#[cfg(feature = "async")]
use tokio::task::spawn_blocking;

use crate::errors::Result;

use self::types::SchemaRelationType;

/// Schema
///
/// This trait is defined through the [`async-trait`](https://crates.io/crates/async-trait) macro.
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait Schema: DeserializeOwned + Serialize + Send + Into<Bson> + Clone {
  type __SchemaId: Into<Bson> + Clone + Send;

  fn __get_database(database: Option<Database>) -> &'static Database;

  fn __get_collection_name() -> String;

  fn __get_id(&self) -> Self::__SchemaId;

  fn __get_id_query(&self) -> Document {
    doc! { "_id": self.__get_id().into() }
  }

  fn __to_document(&self) -> Result<Document> {
    let bson: Bson = self.into();

    match bson.as_document() {
      Some(doc) => Ok(doc.clone()),
      None => unreachable!(),
    }
  }

  fn __check_unique_fields(&self, database: &Database) -> Result<()>;

  fn __relations() -> Vec<types::SchemaRelation>;

  fn __get_relations(&self) -> Option<Vec<types::SchemaRelation>>;

  fn __set_relations(&mut self, field: &str, new_value: Bson) -> Result<()>;

  fn __populate_sync(mut self, field: &str) -> Result<Self> {
    let database = Self::__get_database(None);

    if let Some(relations) = self.__get_relations() {
      for relation in relations.iter() {
        if relation.field_ident == field {
          let collection_name = &relation.schema_name;

          if relation.relation_type == SchemaRelationType::OneToOne
            || relation.relation_type == SchemaRelationType::ManyToOne
          {
            if let Some(data) = database
              .collection::<Document>(collection_name.as_str())
              .find_one(Some(doc! { "_id": relation.field_value.clone() }), None)?
            {
              self.__set_relations(field, Bson::Document(data))?;
            }
          } else if relation.relation_type == SchemaRelationType::OneToMany {
            if let Some(schema) = crate::nongoose::globals::get_schema(collection_name) {
              for schema_relation in schema.get_relations().iter() {
                if schema_relation.relation_type != SchemaRelationType::ManyToOne {
                  continue;
                }

                if schema_relation.schema_name == Self::__get_collection_name() {
                  let documents: Vec<mongodb::error::Result<Document>> = database
                    .collection::<Document>(collection_name.as_str())
                    .find(
                      Some(doc! { schema_relation.field_id(): self.__get_id().into() }),
                      None,
                    )?
                    .collect();

                  let mut data = Vec::new();
                  for doc in documents {
                    let doc = doc?;
                    data.push(doc);
                  }

                  self.__set_relations(field, bson!(data))?;
                  break;
                }
              }
            }
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
