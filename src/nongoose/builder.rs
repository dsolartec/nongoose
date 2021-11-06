use mongodb::{
  bson::{from_bson, Bson, Document},
  options::{FindOneOptions, FindOptions, UpdateOptions},
  results::UpdateResult,
  sync::Database,
};

use crate::{errors::Result, schema::SchemaData, Nongoose, Schema};

#[derive(Clone, Debug)]
pub struct NongooseBuilder {
  pub database: Database,
  pub schemas: Vec<SchemaData>,
}

impl NongooseBuilder {
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

  pub fn has_schema(&self, name: &str) -> bool {
    self.schemas.iter().any(|e| e.get_name().as_str() == name)
  }

  pub fn finish(&self) -> Nongoose {
    Nongoose {
      builder: self.clone(),
    }
  }

  pub fn replace_with(&mut self, new_data: Self) {
    self.database = new_data.database;
    self.schemas = new_data.schemas;
  }

  // Internals
  pub(crate) fn find_sync<T>(
    &self,
    conditions: Document,
    options: Option<FindOptions>,
  ) -> Result<Vec<T>>
  where
    T: core::fmt::Debug + Schema,
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
    T: core::fmt::Debug + Schema,
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
    T: core::fmt::Debug + Schema,
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
