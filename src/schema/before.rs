use mongodb::{bson::Bson, sync::Database};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::Result;

/// Schema before functions
///
/// This trait is defined through the [`async-trait`](https://crates.io/crates/async-trait) macro.
#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
pub trait SchemaBefore: DeserializeOwned + Serialize + Send + Into<Bson> + Clone {
  /// Executes a custom validation before insert the document to the database.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// impl SchemaBefore for User {
  ///   fn before_create(&mut self, _db: &Database) -> Result<()> {
  ///     Ok(())
  ///   }
  /// }
  /// ```
  #[cfg(feature = "sync")]
  fn before_create(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }

  /// Executes a custom validation before insert the document to the database.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// #[async_trait::async_trait]
  /// impl SchemaBefore for User {
  ///   async fn before_create(&mut self, _db: &Database) -> Result<()> {
  ///     Ok(())
  ///   }
  /// }
  /// ```
  #[cfg(feature = "tokio-runtime")]
  async fn before_create(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }

  /// Executes a custom validation before delete the document from the database.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// impl SchemaBefore for User {
  ///   fn before_delete(&mut self, _db: &Database) -> Result<bool> {
  ///     Ok(true)
  ///   }
  /// }
  /// ```
  #[cfg(feature = "sync")]
  fn before_delete(&mut self, _db: &Database) -> Result<bool> {
    Ok(true)
  }

  /// Executes a custom validation before delete the document from the database.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// #[async_trait::async_trait]
  /// impl SchemaBefore for User {
  ///   async fn before_delete(&mut self, _db: &Database) -> Result<bool> {
  ///     Ok(true)
  ///   }
  /// }
  /// ```
  #[cfg(feature = "tokio-runtime")]
  async fn before_delete(&mut self, _db: &Database) -> Result<bool> {
    Ok(true)
  }

  /// Executes a custom validation before replace the document in the database (called on `Schema.save()`).
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// impl SchemaBefore for User {
  ///   fn before_update(&mut self, _db: &Database) -> Result<()> {
  ///     Ok(())
  ///   }
  /// }
  /// ```
  #[cfg(feature = "sync")]
  fn before_update(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }

  /// Executes a custom validation before replace the document in the database (called on `Schema.save()`).
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// #[async_trait::async_trait]
  /// impl SchemaBefore for User {
  ///   async fn before_update(&mut self, _db: &Database) -> Result<()> {
  ///     Ok(())
  ///   }
  /// }
  /// ```
  #[cfg(feature = "tokio-runtime")]
  async fn before_update(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }
}
