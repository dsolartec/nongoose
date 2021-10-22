use mongodb::{
  bson::{from_bson, Bson, Document},
  results::InsertOneResult,
  sync::Database,
};

use crate::{errors::Result, Nongoose, Schema};

#[derive(Clone, Debug)]
pub struct NongooseBuilder {
  pub database: Database,
  pub schemas: Vec<String>,
}

impl NongooseBuilder {
  pub fn add_schema<T>(mut self) -> Self
  where
    T: Schema,
  {
    let collection_name = T::__get_collection_name();

    if !self.schemas.contains(&collection_name) {
      self.schemas.push(collection_name);
      T::__get_instance(Some(self.clone()));
    }

    self
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
  pub(crate) fn find_one_sync<T>(&self, conditions: Document) -> Result<Option<T>>
  where
    T: core::fmt::Debug + Schema,
  {
    let collection_name = T::__get_collection_name();

    if !self.schemas.contains(&collection_name) {
      panic!(
        "Schema is not associated to a Nongoose instance ({})",
        collection_name
      );
    }

    Ok(
      match self
        .database
        .collection::<Document>(collection_name.as_str())
        .find_one(Some(conditions), None)?
      {
        Some(document) => from_bson(Bson::Document(document.clone()))?,
        None => None,
      },
    )
  }

  pub(crate) fn create_sync<T>(&self, data: T) -> Result<InsertOneResult>
  where
    T: Schema,
  {
    let collection_name = T::__get_collection_name();

    if !self.schemas.contains(&collection_name) {
      panic!(
        "Schema is not associated to a Nongoose instance ({})",
        collection_name
      );
    }

    data.__check_unique_fields(&self.database)?;

    Ok(
      self
        .database
        .collection(collection_name.as_str())
        .insert_one(data.__to_document()?, None)?,
    )
  }
}
