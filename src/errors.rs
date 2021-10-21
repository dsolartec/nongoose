#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("BSON decoding error: {0}")]
  BSONDecode(#[from] mongodb::bson::ser::Error),

  #[error("BSON encoding error: {0}")]
  BSONEncode(#[from] mongodb::bson::de::Error),

  #[error("MongoDB error: {0}")]
  MongoDB(#[from] mongodb::error::Error),

  #[cfg(feature = "tokio")]
  #[error("Tokio task error: {0}")]
  Task(#[from] tokio::task::JoinError),

  // Schema Errors
  #[error("Duplicated schema field ({0}): {1}")]
  DuplicatedSchemaField(String, String),

  #[error("No implemented")]
  NoImplemented,
}

pub type Result<T> = std::result::Result<T, Error>;
