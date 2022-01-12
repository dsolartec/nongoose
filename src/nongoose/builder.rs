use mongodb::{
  bson::{from_bson, Bson, Document},
  options::{CountOptions, FindOneOptions, FindOptions, UpdateOptions},
  results::UpdateResult,
  sync::Database,
};

use crate::{error::Result, schema::SchemaData, Nongoose, Schema};

/// Specifies the options to a Nongoose instance.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct NongooseBuilder {
  /// MongoDB instance
  pub database: Database,

  /// Registered schemas.
  ///
  /// You can add one using `NongooseBuilder.add_schema<Schema>()`
  pub schemas: Vec<SchemaData>,
}

impl NongooseBuilder {
  /// Register a Schema if it was not registered before.
  pub fn add_schema<T>(mut self) -> Self
  where
    T: Schema,
  {
    let collection_name = T::__get_collection_name();

    if !self.has_schema(&collection_name) {
      let schema = SchemaData::new::<T>();

      super::globals::add_schema(&schema);
      self.schemas.push(schema);

      T::__get_database(Some(self.database.clone()));
    }

    self
  }

  /// Verify if the Nongoose instance has a registered Schema.
  pub fn has_schema(&self, name: &str) -> bool {
    self.schemas.iter().any(|e| e.get_name().as_str() == name)
  }

  /// Return the Nongoose instance.
  pub fn build(&self) -> Nongoose {
    Nongoose {
      builder: self.clone(),
    }
  }

  // Internals
  pub(crate) fn count_sync<T>(
    &self,
    conditions: Document,
    options: Option<CountOptions>,
  ) -> Result<u64>
  where
    T: Schema,
  {
    let collection_name = T::__get_collection_name();
    if !self.has_schema(&collection_name) {
      panic!(
        "Schema is not associated to a Nongoose instance ({})",
        collection_name
      );
    }

    Ok(
      self
        .database
        .collection::<Document>(collection_name.as_str())
        .count_documents(conditions, options)?,
    )
  }

  pub(crate) fn find_sync<T>(
    &self,
    conditions: Document,
    options: Option<FindOptions>,
  ) -> Result<Vec<T>>
  where
    T: Schema,
  {
    let collection_name = T::__get_collection_name();
    if !self.has_schema(&collection_name) {
      panic!(
        "Schema is not associated to a Nongoose instance ({})",
        collection_name
      );
    }

    let cursor = self
      .database
      .collection::<Document>(collection_name.as_str())
      .find(Some(conditions), options)?;

    let mut documents = Vec::new();
    for doc in cursor.collect::<Vec<mongodb::error::Result<Document>>>() {
      documents.push(from_bson(Bson::Document(doc?))?);
    }

    Ok(documents)
  }

  pub(crate) fn find_one_sync<T>(
    &self,
    conditions: Document,
    options: Option<FindOneOptions>,
  ) -> Result<Option<T>>
  where
    T: Schema,
  {
    let collection_name = T::__get_collection_name();

    if !self.has_schema(&collection_name) {
      panic!(
        "Schema is not associated to a Nongoose instance ({})",
        collection_name
      );
    }

    Ok(
      match self
        .database
        .collection::<Document>(collection_name.as_str())
        .find_one(Some(conditions), options)?
      {
        Some(document) => from_bson(Bson::Document(document))?,
        None => None,
      },
    )
  }

  pub(crate) fn update_many_sync<T>(
    &self,
    conditions: Document,
    data: Document,
    options: Option<UpdateOptions>,
  ) -> Result<UpdateResult>
  where
    T: Schema,
  {
    let collection_name = T::__get_collection_name();

    if !self.has_schema(&collection_name) {
      panic!(
        "Schema is not associated to a Nongoose instance ({})",
        collection_name
      );
    }

    Ok(
      self
        .database
        .collection::<Document>(collection_name.as_str())
        .update_many(conditions, data, options)?,
    )
  }
}
