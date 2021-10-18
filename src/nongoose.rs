mod options;

use futures::TryStreamExt;
use mongodb::{
  bson::{doc, from_bson, Bson, Document},
  results::InsertOneResult,
  Database,
};
pub use options::*;

use crate::{
  errors::{Error, Result},
  Schema,
};

#[derive(Clone)]
pub struct NongooseBuilder {
  pub(crate) database: Database,
}

impl NongooseBuilder {
  pub fn finish(&self) -> Nongoose {
    Nongoose {
      builder: self.clone(),
    }
  }
}

pub struct Nongoose {
  builder: NongooseBuilder,
}

impl Nongoose {
  pub fn build(database: Database) -> NongooseBuilder {
    NongooseBuilder { database }
  }

  /// Finds a single document by its `_id` field. `Nongoose.find_by_id(id)` is almost equivalent to `MongoDB.find_one(doc! { "_id": id })`. If
  /// you want to query by a document's `_id`, use `Nongoose.find_by_id()` instead of `Nongoose.find_one()`.
  ///
  /// This function triggers `MongoDB.aggregate()`.
  ///
  /// # Options
  /// ```rust,no_run
  /// FindByIdOptions::build()
  ///   .with_relations(true) // <-- Parse field relations or no (one to one, many to one, ...)
  /// ```
  ///
  /// # Example
  /// ```rust,no_run
  /// // Find one `User` document by `_id` without relations
  /// match nongoose
  ///   .find_by_id::<User>(
  ///     ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap(),
  ///     None
  ///   )
  ///   .await
  /// {
  ///   Ok(user) => {
  ///     println!("User found: {}", user.id);
  ///   },
  ///   Err(error) => {
  ///     eprintln!("Error finding user: {}", error);
  ///   },
  /// }
  ///
  /// // Find one `User` document by `_id` with relations
  /// match nongoose
  ///   .find_by_id::<User>(
  ///     ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap(),
  ///     Some(FindByIdOptions::build().with_relations(true))
  ///   )
  ///   .await
  /// {
  ///   Ok(user) => {
  ///     println!("User found: {}", user.id);
  ///   },
  ///   Err(error) => {
  ///     eprintln!("Error finding the user: {}", error);
  ///   },
  /// }
  /// ```
  pub async fn find_by_id<T>(
    &self,
    id: T::__SchemaId,
    options: Option<FindByIdOptions>,
  ) -> Result<T>
  where
    T: core::fmt::Debug + Schema,
  {
    let mut pipeline = Vec::new();
    pipeline.push(doc! { "$match": { "_id": id.into() } });

    if let Some(options) = options {
      if options.with_relations {
        if let Some(relations) = T::__get_relations() {
          pipeline = [pipeline, relations].concat();
        }
      }
    }

    let cursor = self
      .builder
      .database
      .collection::<Document>(T::__get_collection_name().as_str())
      .aggregate(pipeline, None)
      .await?;

    let documents: Vec<Document> = cursor.try_collect().await?;
    if documents.len() > 0 {
      return Ok(from_bson(Bson::Document(documents[0].clone()))?);
    }

    Err(Error::NoImplemented)
  }

  /// Save one document to the database.
  ///
  /// # Example
  /// ```rust,no_run
  /// // Insert one new `User` document
  /// match nongoose
  ///   .create::<User>(&user)
  ///   .await
  /// {
  ///   Ok(result) => {
  ///     println!("User saved: {}", result.inserted_id);
  ///   },
  ///   Err(error) => {
  ///     eprintln!("Error saving the user: {}", error);
  ///   }
  /// }
  /// ```
  pub async fn create<T>(&self, data: &T) -> Result<InsertOneResult>
  where
    T: Schema,
  {
    data.__check_unique_fields(&self.builder.database).await?;

    Ok(
      self
        .builder
        .database
        .collection(T::__get_collection_name().as_str())
        .insert_one(data.__to_document()?, None)
        .await?,
    )
  }
}
