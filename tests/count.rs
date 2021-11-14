use nongoose::{
  mongodb::{
    bson::{doc, oid::ObjectId, Regex},
    sync::Client,
  },
  Nongoose, Schema, SchemaBefore,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
#[schema(name = "actors_count")]
struct Actor {
  #[schema(id)]
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub fullname: String,
  pub age: u64,
}

impl Actor {
  pub fn new(fullname: &str, age: u64) -> Self {
    Self {
      id: ObjectId::new(),
      fullname: String::from(fullname),
      age,
    }
  }
}

#[cfg_attr(feature = "async", async_trait::async_trait)]
impl SchemaBefore for Actor {}

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
    .add_schema::<Actor>()
    .finish()
}

#[cfg(not(feature = "async"))]
#[cfg_attr(not(feature = "async"), test)]
fn count() {
  let nongoose = get_instance();

  let tom_hanks = Actor::new("Tom Hanks", 65).save();
  assert!(tom_hanks.is_ok());

  let will_smith = Actor::new("Will Smith", 53).save();
  assert!(will_smith.is_ok());

  let leonardo_dicaprio = Actor::new("Leonardo DiCaprio", 46).save();
  assert!(leonardo_dicaprio.is_ok());

  let jeniffer_lopez = Actor::new("Jeniffer Lopez", 52).save();
  assert!(jeniffer_lopez.is_ok());

  let tom_cruise = Actor::new("Tom Cruise", 59).save();
  assert!(tom_cruise.is_ok());

  let emma_stone = Actor::new("Emma Stone", 32).save();
  assert!(emma_stone.is_ok());

  // Count actors between 40 and 49 years old
  let actors_40s_age = nongoose.count::<Actor>(doc! { "age": { "$gte": 40, "$lte": 49 } }, None);
  assert!(actors_40s_age.is_ok());
  assert_eq!(actors_40s_age.unwrap(), 1_u64);

  // Count actors between 50 and 59 years old
  let actors_50s_age = nongoose.count::<Actor>(doc! { "age": { "$gte": 50, "$lte": 59 }}, None);
  assert!(actors_50s_age.is_ok());
  assert_eq!(actors_50s_age.unwrap(), 3_u64);

  // Count actors with the word "Tom" in their name
  let tom_actors = nongoose.count::<Actor>(
    doc! { "fullname": Regex { pattern: String::from("^Tom"), options: String::new() } },
    None,
  );
  assert!(tom_actors.is_ok());
  assert_eq!(tom_actors.unwrap(), 2_u64);
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "async", tokio::test)]
async fn count() {
  let nongoose = get_instance();

  let tom_hanks = Actor::new("Tom Hanks", 65).save().await;
  assert!(tom_hanks.is_ok());

  let will_smith = Actor::new("Will Smith", 53).save().await;
  assert!(will_smith.is_ok());

  let leonardo_dicaprio = Actor::new("Leonardo DiCaprio", 46).save().await;
  assert!(leonardo_dicaprio.is_ok());

  let jeniffer_lopez = Actor::new("Jeniffer Lopez", 52).save().await;
  assert!(jeniffer_lopez.is_ok());

  let tom_cruise = Actor::new("Tom Cruise", 59).save().await;
  assert!(tom_cruise.is_ok());

  let emma_stone = Actor::new("Emma Stone", 32).save().await;
  assert!(emma_stone.is_ok());

  // Count actors between 40 and 49 years old
  let actors_40s_age = nongoose
    .count::<Actor>(doc! { "age": { "$gte": 40, "$lte": 49 } }, None)
    .await;
  assert!(actors_40s_age.is_ok());
  assert_eq!(actors_40s_age.unwrap(), 1_u64);

  // Count actors between 50 and 59 years old
  let actors_50s_age = nongoose
    .count::<Actor>(doc! { "age": { "$gte": 50, "$lte": 59 }}, None)
    .await;
  assert!(actors_50s_age.is_ok());
  assert_eq!(actors_50s_age.unwrap(), 3_u64);

  // Count actors with the word "Tom" in their name
  let tom_actors = nongoose
    .count::<Actor>(
      doc! { "fullname": Regex { pattern: String::from("^Tom"), options: String::new() } },
      None,
    )
    .await;
  assert!(tom_actors.is_ok());
  assert_eq!(tom_actors.unwrap(), 2_u64);
}
