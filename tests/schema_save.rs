use nongoose::{bson::oid::ObjectId, Client, Nongoose, Schema, SchemaBefore};
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
  #[schema(unique)]
  pub name: String,
}

#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
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

  Nongoose::builder(client.database("nongoose"))
    .add_schema::<Animal>()
    .build()
}

#[cfg(feature = "sync")]
#[cfg_attr(feature = "sync", test)]
fn schema_save() {
  let _nongoose = get_instance();

  let mut dog = Animal::new(AnimalType::Dog, "dog");

  let dog_saved = dog.save();
  assert!(dog_saved.is_ok());

  let mut dog_saved = dog_saved.unwrap();
  assert_eq!(dog, dog_saved);

  let duplicated_dog = Animal::new(AnimalType::Dog, "dog").save();
  assert!(duplicated_dog.is_err());
  assert_eq!(
    format!("{}", duplicated_dog.unwrap_err()),
    String::from("Duplicated schema field (name): dog")
  );

  dog_saved.name = String::from("dog1");
  assert!(dog_saved.save().is_ok());

  dog_saved.name = String::from("dog1");
  assert!(dog_saved.save().is_ok());

  let mut cat = Animal::new(AnimalType::Cat, "cat");

  let cat_saved = cat.save();
  assert!(cat_saved.is_ok());
  assert_eq!(cat, cat_saved.unwrap());
}

#[cfg(feature = "tokio-runtime")]
#[cfg_attr(feature = "tokio-runtime", tokio::test)]
async fn schema_save() {
  let _nongoose = get_instance();

  let mut dog = Animal::new(AnimalType::Dog, "dog");

  let dog_saved = dog.save().await;
  assert!(dog_saved.is_ok());

  let mut dog_saved = dog_saved.unwrap();
  assert_eq!(dog, dog_saved);

  let duplicated_dog = Animal::new(AnimalType::Dog, "dog").save().await;
  assert!(duplicated_dog.is_err());
  assert_eq!(
    format!("{}", duplicated_dog.unwrap_err()),
    String::from("Duplicated schema field (name): dog")
  );

  dog_saved.name = String::from("dog1");
  assert!(dog_saved.save().await.is_ok());

  dog_saved.name = String::from("dog1");
  assert!(dog_saved.save().await.is_ok());

  let mut cat = Animal::new(AnimalType::Cat, "cat");
  let cat_saved = cat.save().await;

  assert!(cat_saved.is_ok());
  assert_eq!(cat, cat_saved.unwrap());
}
