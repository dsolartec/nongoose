mod data;
pub mod types;

pub use data::SchemaData;
use mongodb::{
  bson::{bson, doc, Bson, Document},
  options::ReplaceOptions,
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
  type Id: Into<Bson> + Clone + Send;

  #[doc(hidden)]
  fn __get_database(database: Option<Database>) -> &'static Database;

  #[doc(hidden)]
  fn __get_collection_name() -> String;

  #[doc(hidden)]
  fn __get_id(&self) -> Self::Id;

  #[doc(hidden)]
  fn __get_id_query(&self) -> Document {
    doc! { "_id": self.__get_id().into() }
  }

  #[doc(hidden)]
  fn __to_document(&self) -> Result<Document> {
    let bson: Bson = self.into();

    match bson.as_document() {
      Some(doc) => Ok(doc.clone()),
      None => unreachable!(),
    }
  }

  #[doc(hidden)]
  fn __check_unique_fields(&self) -> Result<()>;

  #[doc(hidden)]
  fn __relations() -> Vec<types::SchemaRelation>;

  #[doc(hidden)]
  fn __get_relations(&self) -> Option<Vec<types::SchemaRelation>>;

  #[doc(hidden)]
  fn __set_relations(&mut self, field: &str, new_value: Bson) -> Result<()>;

  #[doc(hidden)]
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

  #[doc(hidden)]
  fn __save_sync(self) -> Result<Self> {
    let database = Self::__get_database(None);
    let collection = database.collection::<Document>(Self::__get_collection_name().as_str());

    self.__check_unique_fields()?;

    if collection
      .find_one(Some(self.__get_id_query()), None)?
      .is_some()
    {
      collection.replace_one(
        self.__get_id_query(),
        self.__to_document()?,
        Some(ReplaceOptions::builder().upsert(true).build()),
      )?;
    } else {
      collection.insert_one(self.__to_document()?, None)?;
    }

    Ok(self)
  }

  #[cfg(not(feature = "async"))]
  fn save(self) -> Result<Self> {
    self.__save_sync()
  }

  #[cfg(feature = "async")]
  async fn save(self) -> Result<Self>
  where
    Self: 'static,
  {
    spawn_blocking(|| self.__save_sync()).await?
  }
}
