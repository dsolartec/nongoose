use mongodb::{
  bson::oid::ObjectId,
  sync::{Client, Database},
};
use nongoose::{Nongoose, Schema, SchemaBefore};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
struct User {
  #[schema(id, unique)]
  #[serde(rename = "_id")]
  pub id: ObjectId,
  #[schema(unique)]
  pub username: String,
  pub password: String,
}

impl User {
  pub fn new(username: &str, password: &str) -> Self {
    Self {
      id: ObjectId::new(),
      username: String::from(username),
      password: String::from(password),
    }
  }

  fn change_password(&mut self) {
    self.password = self
      .password
      .chars()
      .map(|c| {
        let case = if c.is_uppercase() { 'A' } else { 'a' } as u8;
        if c.is_alphabetic() {
          (((c as u8 - case + 3) % 26) + case) as char
        } else {
          c
        }
      })
      .collect::<String>();
  }
}

#[cfg_attr(feature = "async", async_trait::async_trait)]
impl SchemaBefore for User {
  #[cfg(not(feature = "async"))]
  fn before_create(&mut self, _db: &Database) -> nongoose::errors::Result<()> {
    self.change_password();
    Ok(())
  }

  #[cfg(feature = "async")]
  async fn before_create(&mut self, _db: &Database) -> nongoose::errors::Result<()> {
    self.change_password();
    Ok(())
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
    .add_schema::<User>()
    .finish()
}

#[cfg(not(feature = "async"))]
#[test]
fn before_create() {
  let nongoose = get_instance();

  let user = User::new("nongoose", "password").save();
  assert!(user.is_ok());

  let user = user.unwrap();
  assert_eq!(user.password, String::from("sdvvzrug"));

  let check_user = nongoose.find_by_id::<User>(&user.id);
  assert!(check_user.is_ok());

  let check_user = check_user.unwrap();
  assert!(check_user.is_some());

  let check_user = check_user.unwrap();
  assert_eq!(check_user.password, user.password);
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "async", tokio::test)]
async fn before_create() {
  let nongoose = get_instance();

  let user = User::new("nongoose", "password").save().await;
  assert!(user.is_ok());

  let user = user.unwrap();
  assert_eq!(user.password, String::from("sdvvzrug"));

  let check_user = nongoose.find_by_id::<User>(&user.id).await;
  assert!(check_user.is_ok());

  let check_user = check_user.unwrap();
  assert!(check_user.is_some());

  let check_user = check_user.unwrap();
  assert_eq!(check_user.password, user.password);
}
