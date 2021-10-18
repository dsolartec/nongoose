use mongodb::{bson::oid::ObjectId, Client};
use nongoose::{FindByIdOptions, Schema};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
struct User {
  #[schema(id)]
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[schema(unique)]
  pub username: String,
}

#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
struct UserFriend {
  #[schema(id)]
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[schema(many_to_one = "User")]
  #[serde(skip_serializing)]
  pub from: Option<User>,

  #[schema(many_to_one = "User")]
  #[serde(skip_serializing)]
  pub to: Option<User>,
}

#[tokio::main]
async fn main() {
  // Get database url.
  let url = match std::env::var("DATABASE_URL") {
    Ok(url) => url,
    Err(_) => {
      panic!("Cannot find `DATABASE_URL` on the environment variables.");
    }
  };

  // Get MongoDB connection.
  let client = match Client::with_uri_str(&url).await {
    Ok(client) => client,
    Err(e) => {
      panic!("Error connecting to the database: {}", e);
    }
  };

  let nongoose = nongoose::Nongoose::build(client.database("nextchat")).finish();

  let user_friend_id = ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap();
  let user_friends = nongoose
    .find_by_id::<UserFriend>(
      user_friend_id,
      Some(FindByIdOptions::build().with_relations(true)),
    )
    .await;

  if let Err(e) = user_friends.clone() {
    println!("Error finding the user friends: {}", e);

    let user_one = User {
      id: ObjectId::new(),
      username: String::from("nongoose"),
    };

    if let Err(e) = nongoose.create(&user_one).await {
      eprintln!("Error creating user one: {}", e);
      return;
    }

    let user_two = User {
      id: ObjectId::new(),
      username: String::from("nongoose2"),
    };

    if let Err(e) = nongoose.create(&user_two).await {
      eprintln!("Error creating user two: {}", e);
      return;
    }

    let user_friend = UserFriend {
      id: user_friend_id,
      from: Some(user_one),
      to: Some(user_two),
    };

    if let Err(e) = nongoose.create(&user_friend).await {
      eprintln!("Error creating user friends: {}", e);
      return;
    }
  }

  let data = user_friends.unwrap();
  println!("User friend: {:?}", data);
}
