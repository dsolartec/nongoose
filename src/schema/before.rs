use mongodb::{bson::Bson, sync::Database};
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::Result;

/// Schema before functions
///
/// This trait is defined through the [`async-trait`](https://crates.io/crates/async-trait) macro.
#[cfg_attr(feature = "async", async_trait::async_trait)]
pub trait SchemaBefore: DeserializeOwned + Serialize + Send + Into<Bson> + Clone {
  #[cfg(not(feature = "async"))]
  fn before_create(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }

  #[cfg(feature = "async")]
  async fn before_create(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }

  #[cfg(not(feature = "async"))]
  fn before_update(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }

  #[cfg(feature = "async")]
  async fn before_update(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }
}
