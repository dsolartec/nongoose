mod before;
mod data;

/// Schema types:
///
/// - Type of the relation.
/// - Data of the relation.
pub mod types;

pub use before::SchemaBefore;
pub use data::SchemaData;
use mongodb::{
  bson::{bson, doc, Bson, Document},
  options::ReplaceOptions,
  sync::Database,
};
#[cfg(feature = "tokio-runtime")]
use tokio::task::spawn_blocking;

use crate::error::Result;

use self::types::SchemaRelationType;

/// Schema
///
/// This trait is defined through the [`async-trait`](https://crates.io/crates/async-trait) macro.
#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
pub trait Schema: SchemaBefore {
  /// `_id` field of the Document.
  ///
  /// In the Schema is defined as `#[schema(id)]`
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
  fn __populate_sync(&mut self, field: &str) -> Result<Self> {
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

    Ok(self.clone())
  }

  /// Populates fields on an existing schema.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Populate the role of the user
  /// match user.clone().populate("role") {
  ///   Ok(u) => user = u,
  ///   Err(error) => eprintln!("Error populating user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "sync")]
  fn populate(mut self, field: &str) -> Result<Self> {
    self.__populate_sync(field)
  }

  /// Populates fields on an existing schema.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Populate the role of the user
  /// match user.clone().populate("role").await {
  ///   Ok(u) => user = u,
  ///   Err(error) => eprintln!("Error populating user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "tokio-runtime")]
  async fn populate(mut self, field: &'static str) -> Result<Self>
  where
    Self: 'static,
  {
    spawn_blocking(move || self.__populate_sync(field)).await?
  }

  /// Removes this document from the db.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// match user.remove() {
  ///   Ok(true) => println!("The user was deleted!"),
  ///   Ok(false) => println!("The user could not be deleted!"),
  ///   Err(error) => eprintln!("Error deleting the user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "sync")]
  fn remove(&self) -> Result<bool> {
    let db = Self::__get_database(None);
    let collection = db.collection::<Document>(Self::__get_collection_name().as_str());

    let result = collection.delete_one(self.__get_id_query(), None)?;
    Ok(result.deleted_count == 1)
  }

  /// Removes this document from the db.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// match user.remove().await {
  ///   Ok(true) => println!("The user was deleted!"),
  ///   Ok(false) => println!("The user could not be deleted!"),
  ///   Err(error) => eprintln!("Error deleting the user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "tokio-runtime")]
  async fn remove(&self) -> Result<bool> {
    let db = Self::__get_database(None);
    let collection = db.collection::<Document>(Self::__get_collection_name().as_str());

    let id = self.__get_id_query();

    let result = spawn_blocking(move || collection.delete_one(id, None)).await??;
    Ok(result.deleted_count == 1)
  }

  /// Saves this document by inserting a new document into the database if it does not exist before, or sends an `replace_one` operation with the modifications to the database.
  ///
  /// If the document needs to be inserted to the database, the `SchemaBefore.before_create()` method is called before insert the document;
  /// otherwise, `SchemaBefore.before_update()` is called before replace the document.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// match user.save() {
  ///   Ok(u) => user = u,
  ///   Err(error) => eprintln!("Error saving user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "sync")]
  fn save(&mut self) -> Result<Self> {
    let db = Self::__get_database(None);
    let collection = db.collection::<Document>(Self::__get_collection_name().as_str());

    self.__check_unique_fields()?;

    if collection
      .find_one(Some(self.__get_id_query()), None)?
      .is_some()
    {
      self.before_update(db)?;

      let id_query = self.__get_id_query();
      let document = self.__to_document()?;

      collection.replace_one(
        id_query,
        document,
        Some(ReplaceOptions::builder().upsert(true).build()),
      )?;
    } else {
      self.before_create(db)?;

      let document = self.__to_document()?;
      collection.insert_one(document, None)?;
    }

    Ok(self.clone())
  }

  /// Saves this document by inserting a new document into the database if it does not exist before, or sends an `replace_one` operation with the modifications to the database.
  ///
  /// If the document needs to be inserted to the database, the `SchemaBefore.before_create()` method is called before insert the document;
  /// otherwise, `SchemaBefore.before_update()` is called before replace the document.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// match user.save().await {
  ///   Ok(u) => user = u,
  ///   Err(error) => eprintln!("Error saving user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "tokio-runtime")]
  async fn save(&mut self) -> Result<Self> {
    let db = Self::__get_database(None);
    let collection = db.collection::<Document>(Self::__get_collection_name().as_str());

    self.__check_unique_fields()?;

    let id_query = self.__get_id_query();
    let result = spawn_blocking(move || collection.find_one(Some(id_query), None)).await??;

    if result.is_some() {
      self.before_update(db).await?;

      let collection = db.collection::<Document>(Self::__get_collection_name().as_str());

      let id_query = self.__get_id_query();
      let document = self.__to_document()?;

      spawn_blocking(move || {
        collection.replace_one(
          id_query,
          document,
          Some(ReplaceOptions::builder().upsert(true).build()),
        )
      })
      .await??;
    } else {
      self.before_create(db).await?;

      let collection = db.collection::<Document>(Self::__get_collection_name().as_str());

      let document = self.__to_document()?;
      spawn_blocking(move || collection.insert_one(document, None)).await??;
    }

    Ok(self.clone())
  }
}
