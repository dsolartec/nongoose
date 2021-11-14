use mongodb::{bson::oid::ObjectId, sync::Client};
use nongoose::{Nongoose, Schema, SchemaBefore};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnimalType {
  Cat,
  Dog,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Schema, Serialize)]
pub struct Animal {
  #[schema(id, unique)]
  #[serde(rename = "_id")]
  pub id: ObjectId,
  #[serde(rename = "type")]
  pub animal_type: AnimalType,
  pub name: String,
}

#[cfg_attr(feature = "async", async_trait::async_trait)]
impl SchemaBefore for Animal {}

impl Animal {
  pub fn new(animal_type: AnimalType, name: &str) -> Self {
    Self {
      id: ObjectId::new(),
      animal_type,
      name: String::from(name),
    }
  }
}

#[cfg(test)]
fn get_instance() -> Nongoose {
  // Get database url.
  let url = match std::env::var("DATABASE_URL") {
    Ok(url) => url,
    Err(_) => {
      panic!("Cannot find `DATABASE_URL` on the environment variables.");
    }
  };

  // Get MongoDB connection.
  let client = match Client::with_uri_str(&url) {
    Ok(client) => client,
    Err(e) => {
      panic!("Error connecting to the database: {}", e);
    }
  };

  Nongoose::build(client.database("nongoose"))
    .add_schema::<Animal>()
    .finish()
}

#[cfg(not(feature = "async"))]
#[cfg_attr(not(feature = "async"), test)]
fn schema_remove() {
  let nongoose = get_instance();

  let mut dog = Animal::new(AnimalType::Dog, "dog");

  let dog_saved = dog.save();
  assert!(dog_saved.is_ok());

  let dog_saved = dog_saved.unwrap();
  assert_eq!(dog, dog_saved);

  let dog_removed = dog_saved.remove();
  assert!(dog_removed.is_ok());
  assert!(dog_removed.unwrap());

  let dog_by_id = nongoose.find_by_id::<Animal>(&dog.id);
  assert!(dog_by_id.is_ok());
  assert!(dog_by_id.unwrap().is_none());
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "async", tokio::test)]
async fn schema_remove() {
  let nongoose = get_instance();

  let mut dog = Animal::new(AnimalType::Dog, "dog");

  let dog_saved = dog.save().await;
  assert!(dog_saved.is_ok());

  let dog_saved = dog_saved.unwrap();
  assert_eq!(dog, dog_saved);

  let dog_removed = dog_saved.remove().await;
  assert!(dog_removed.is_ok());
  assert!(dog_removed.unwrap());

  let dog_by_id = nongoose.find_by_id::<Animal>(&dog.id).await;
  assert!(dog_by_id.is_ok());
  assert!(dog_by_id.unwrap().is_none());
}
