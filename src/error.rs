/// An error that can occur in the `nongoose` crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Wrap MongoDB BSON deconding errors.
  #[error("BSON decoding error: {0}")]
  BSONDecode(#[from] mongodb::bson::de::Error),

  /// Wrap MongoDB BSON enconding errors.
  #[error("BSON encoding error: {0}")]
  BSONEncode(#[from] mongodb::bson::ser::Error),

  /// Wrap MongoDB errors.
  #[error("MongoDB error: {0}")]
  MongoDB(#[from] mongodb::error::Error),

  /// Wrap BSON Document value access errors.
  #[error("Document value access error")]
  DocumentAccessError(#[from] mongodb::bson::document::ValueAccessError),

  /// Wrap Tokio Task errors.
  #[cfg(feature = "tokio")]
  #[error("Tokio task error: {0}")]
  Task(#[from] tokio::task::JoinError),

  /// Wrap duplicated schema field (`field_name` and `field_value`).
  #[error("Duplicated schema field ({0}): {1}")]
  DuplicatedSchemaField(String, String),

  /// Wrap no implemented errors.
  #[error("No implemented")]
  NoImplemented,
}

/// The result type for all methods that can return an error in the `nongoose` crate.
pub type Result<T> = std::result::Result<T, Error>;
