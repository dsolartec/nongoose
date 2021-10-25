mod builder;
pub(crate) mod globals;

pub use builder::NongooseBuilder;
use mongodb::{
  bson::{doc, Document},
  sync::Database,
};
#[cfg(feature = "async")]
use tokio::task::spawn_blocking;

use crate::{errors::Result, Schema};

#[derive(Clone)]
pub struct Nongoose {
  builder: NongooseBuilder,
}

impl Nongoose {
  pub fn build(database: Database) -> NongooseBuilder {
    NongooseBuilder {
      database,
      schemas: Vec::new(),
    }
  }

  /// Finds one document.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Find one user whose `username` is `nongoose`
  /// match nongoose.find_one::<User>(doc! { "username": "nongoose" }) {
  ///   Ok(Some(user)) => println!("User found: {}", user.id),
  ///   Ok(None) => eprintln!("Cannot find the user"),
  ///   Err(error) => eprintln!("Error finding user: {}", error),
  /// }
  /// ```
  #[cfg(not(feature = "async"))]
  pub fn find_one<T>(&self, conditions: Document) -> Result<Option<T>>
  where
    T: core::fmt::Debug + Schema,
  {
    self.builder.find_one_sync(conditions)
  }

  /// Finds one document.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Find one user whose `username` is `nongoose`
  /// match nongoose.find_one::<User>(doc! { "username": "nongoose" }).await {
  ///   Ok(Some(user)) => println!("User found: {}", user.id),
  ///   Ok(None) => eprintln!("Cannot find the user"),
  ///   Err(error) => eprintln!("Error finding user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "async")]
  pub async fn find_one<T>(&self, conditions: Document) -> Result<Option<T>>
  where
    T: core::fmt::Debug + Schema + 'static,
  {
    let builder = self.builder.clone();
    spawn_blocking(move || builder.find_one_sync(conditions)).await?
  }

  /// Finds a single document by its `_id` field. `find_by_id(id)`is almost equivalent to `find_one(doc! { "_id": id })`.
  /// If you want to query by a document's `_id`, use `find_by_id()`instead of `find_one()`.
  ///
  /// This function triggers `find_one()`.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Find one `User` document by `_id`
  /// match nongoose.find_by_id::<User>(&ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap()) {
  ///   Ok(Some(user)) => println!("User found: {}", user.id),
  ///   Ok(None) => eprintln!("Cannot find the user"),
  ///   Err(error) => eprintln!("Error finding user: {}", error),
  /// }
  /// ```
  #[cfg(not(feature = "async"))]
  pub fn find_by_id<T>(&self, id: &T::Id) -> Result<Option<T>>
  where
    T: core::fmt::Debug + Schema,
  {
    self.find_one(doc! { "_id": id.clone().into() })
  }

  /// Finds a single document by its `_id` field. `find_by_id(id)`is almost equivalent to `find_one(doc! { "_id": id })`.
  /// If you want to query by a document's `_id`, use `find_by_id()`instead of `find_one()`.
  ///
  /// This function triggers `find_one()`.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Find one `User` document by `_id`
  /// match nongoose.find_by_id::<User>(&ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap()).await {
  ///   Ok(Some(user)) => println!("User found: {}", user.id),
  ///   Ok(None) => eprintln!("Cannot find the user"),
  ///   Err(error) => eprintln!("Error finding user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "async")]
  pub async fn find_by_id<T>(&self, id: &T::Id) -> Result<Option<T>>
  where
    T: core::fmt::Debug + Schema + 'static,
  {
    self.find_one(doc! { "_id": id.clone().into() }).await
  }

  /// Shortcut for saving one document to the database. `Nongoose.create(doc)` does `doc.save()`.
  ///
  /// This function triggers `save()`.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Insert one new `User` document
  /// match nongoose.create::<User>(&user) {
  ///   Ok(user) => {
  ///     println!("User saved: {}", user.id);
  ///   },
  ///   Err(error) => {
  ///     eprintln!("Error saving the user: {}", error);
  ///   }
  /// }
  /// ```
  #[cfg(not(feature = "async"))]
  pub fn create<T>(&self, data: &T) -> Result<T>
  where
    T: Schema + Clone,
  {
    data.clone().save()
  }

  /// Shortcut for saving one document to the database. `Nongoose.create(doc)` does `doc.save()`.
  ///
  /// This function triggers `save()`.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Insert one new `User` document
  /// match nongoose.create::<User>(&user).await {
  ///   Ok(user) => {
  ///     println!("User saved: {}", user.id);
  ///   },
  ///   Err(error) => {
  ///     eprintln!("Error saving the user: {}", error);
  ///   }
  /// }
  /// ```
  #[cfg(feature = "async")]
  pub async fn create<T>(&self, data: &T) -> Result<T>
  where
    T: Schema + Clone + Send + 'static,
  {
    data.clone().save().await
  }
}
