use nongoose::{
  bson::{doc, oid::ObjectId, Regex},
  options::FindOptions,
  Client, Nongoose, Schema, SchemaBefore,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
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

#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
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

  Nongoose::builder(client.database("nongoose"))
    .add_schema::<Actor>()
    .build()
}

#[cfg(feature = "sync")]
#[cfg_attr(feature = "sync", test)]
fn find() {
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

  // Unwrap actors
  let tom_hanks = tom_hanks.unwrap();
  let will_smith = will_smith.unwrap();
  let leonardo_dicaprio = leonardo_dicaprio.unwrap();
  let jeniffer_lopez = jeniffer_lopez.unwrap();
  let tom_cruise = tom_cruise.unwrap();
  let emma_stone = emma_stone.unwrap();

  // Find by id
  let by_id = nongoose.find_by_id::<Actor>(&emma_stone.id);
  assert!(by_id.is_ok());

  let by_id = by_id.unwrap();
  assert!(by_id.is_some());

  let by_id = by_id.unwrap();
  assert_eq!(by_id.fullname, emma_stone.fullname);

  // Find one by age
  let age_32 = nongoose.find_one::<Actor>(doc! { "age": 32 }, None);
  assert!(age_32.is_ok());

  let age_32 = age_32.unwrap();
  assert!(age_32.is_some());

  let age_32 = age_32.unwrap();
  assert_eq!(age_32.fullname, emma_stone.fullname);

  // Find actors between 40 and 49 years old
  let actors_40s_age = nongoose.find::<Actor>(doc! { "age": { "$gte": 40, "$lte": 49 } }, None);
  assert!(actors_40s_age.is_ok());

  let actors_40s_age = actors_40s_age.unwrap();
  assert_eq!(actors_40s_age.len(), 1);
  assert_eq!(actors_40s_age[0].fullname, leonardo_dicaprio.fullname);

  // Find actors between 50 and 59 years old
  let actors_50s_age = nongoose.find::<Actor>(
    doc! { "age": { "$gte": 50, "$lte": 59 }},
    Some(FindOptions::builder().sort(doc! { "age": 1 }).build()),
  );
  assert!(actors_50s_age.is_ok());

  let actors_50s_age = actors_50s_age.unwrap();
  assert_eq!(actors_50s_age.len(), 3);
  assert_eq!(actors_50s_age[0].fullname, jeniffer_lopez.fullname);
  assert_eq!(actors_50s_age[1].fullname, will_smith.fullname);
  assert_eq!(actors_50s_age[2].fullname, tom_cruise.fullname);

  // Find actors with the word "Tom" in their name
  let tom_actors = nongoose.find::<Actor>(
    doc! { "fullname": Regex { pattern: String::from("^Tom"), options: String::new() } },
    Some(FindOptions::builder().sort(doc! { "age": -1 }).build()),
  );
  assert!(tom_actors.is_ok());

  let tom_actors = tom_actors.unwrap();
  assert_eq!(tom_actors.len(), 2);
  assert_eq!(tom_actors[0].fullname, tom_hanks.fullname);
  assert_eq!(tom_actors[1].fullname, tom_cruise.fullname);
}

#[cfg(feature = "tokio-runtime")]
#[cfg_attr(feature = "tokio-runtime", tokio::test)]
async fn find() {
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

  // Unwrap actors
  let tom_hanks = tom_hanks.unwrap();
  let will_smith = will_smith.unwrap();
  let leonardo_dicaprio = leonardo_dicaprio.unwrap();
  let jeniffer_lopez = jeniffer_lopez.unwrap();
  let tom_cruise = tom_cruise.unwrap();
  let emma_stone = emma_stone.unwrap();

  // Find by id
  let by_id = nongoose.find_by_id::<Actor>(&emma_stone.id).await;
  assert!(by_id.is_ok());

  let by_id = by_id.unwrap();
  assert!(by_id.is_some());

  let by_id = by_id.unwrap();
  assert_eq!(by_id.fullname, emma_stone.fullname);

  // Find one by age
  let age_32 = nongoose.find_one::<Actor>(doc! { "age": 32 }, None).await;
  assert!(age_32.is_ok());

  let age_32 = age_32.unwrap();
  assert!(age_32.is_some());

  let age_32 = age_32.unwrap();
  assert_eq!(age_32.fullname, emma_stone.fullname);

  // Find actors between 40 and 49 years old
  let actors_40s_age = nongoose
    .find::<Actor>(doc! { "age": { "$gte": 40, "$lte": 49 } }, None)
    .await;
  assert!(actors_40s_age.is_ok());

  let actors_40s_age = actors_40s_age.unwrap();
  assert_eq!(actors_40s_age.len(), 1);
  assert_eq!(actors_40s_age[0].fullname, leonardo_dicaprio.fullname);

  // Find actors between 50 and 59 years old
  let actors_50s_age = nongoose
    .find::<Actor>(
      doc! { "age": { "$gte": 50, "$lte": 59 }},
      Some(FindOptions::builder().sort(doc! { "age": 1 }).build()),
    )
    .await;
  assert!(actors_50s_age.is_ok());

  let actors_50s_age = actors_50s_age.unwrap();
  assert_eq!(actors_50s_age.len(), 3);
  assert_eq!(actors_50s_age[0].fullname, jeniffer_lopez.fullname);
  assert_eq!(actors_50s_age[1].fullname, will_smith.fullname);
  assert_eq!(actors_50s_age[2].fullname, tom_cruise.fullname);

  // Find actors with the word "Tom" in their name
  let tom_actors = nongoose
    .find::<Actor>(
      doc! { "fullname": Regex { pattern: String::from("^Tom"), options: String::new() } },
      Some(FindOptions::builder().sort(doc! { "age": -1 }).build()),
    )
    .await;
  assert!(tom_actors.is_ok());

  let tom_actors = tom_actors.unwrap();
  assert_eq!(tom_actors.len(), 2);
  assert_eq!(tom_actors[0].fullname, tom_hanks.fullname);
  assert_eq!(tom_actors[1].fullname, tom_cruise.fullname);
}
