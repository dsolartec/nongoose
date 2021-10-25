use nongoose::{
  mongodb::{bson::oid::ObjectId, sync::Client},
  schema_relations, Nongoose, Schema, SchemaBefore,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
struct User {
  #[schema(id)]
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[schema(unique)]
  pub username: String,
}

#[cfg_attr(feature = "async", async_trait::async_trait)]
impl SchemaBefore for User {}

#[schema_relations]
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

#[cfg_attr(feature = "async", async_trait::async_trait)]
impl SchemaBefore for UserFriend {}

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
    .add_schema::<UserFriend>()
    .finish()
}

#[cfg(not(feature = "async"))]
fn run_sync(nongoose: Nongoose, user_friend_id: &ObjectId) -> nongoose::errors::Result<UserFriend> {
  let user_friends = nongoose.find_by_id::<UserFriend>(user_friend_id);

  match user_friends {
    Ok(None) | Err(_) => {
      let user_one = User {
        id: ObjectId::new(),
        username: String::from("nongoose"),
      };

      nongoose.create(&user_one)?;

      let user_two = User {
        id: ObjectId::new(),
        username: String::from("nongoose2"),
      };

      nongoose.create(&user_two)?;

      let user_friend = UserFriend {
        id: *user_friend_id,
        from: Some(user_one.clone()),
        from_id: user_one.id,
        to: Some(user_two.clone()),
        to_id: user_two.id,
      };

      nongoose.create(&user_friend)?;
      Ok(user_friend)
    }
    Ok(Some(user_friends)) => Ok(user_friends.populate("from")?.populate("to")?),
  }
}

#[cfg(feature = "async")]
async fn run_async(
  nongoose: Nongoose,
  user_friend_id: &ObjectId,
) -> nongoose::errors::Result<UserFriend> {
  let user_friends = nongoose.find_by_id::<UserFriend>(user_friend_id).await;

  match user_friends {
    Ok(None) | Err(_) => {
      let user_one = User {
        id: ObjectId::new(),
        username: String::from("nongoose"),
      };

      nongoose.create(&user_one).await?;

      let user_two = User {
        id: ObjectId::new(),
        username: String::from("nongoose2"),
      };

      nongoose.create(&user_two).await?;

      let user_friend = UserFriend {
        id: *user_friend_id,
        from: Some(user_one.clone()),
        from_id: user_one.id,
        to: Some(user_two.clone()),
        to_id: user_two.id,
      };

      nongoose.create(&user_friend).await?;
      Ok(user_friend)
    }
    Ok(Some(user_friends)) => Ok(user_friends.populate("from").await?.populate("to").await?),
  }
}

#[cfg(not(feature = "async"))]
fn main() -> nongoose::errors::Result<()> {
  let user_friend_id = ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap();
  let nongoose = get_instance();

  let data = run_sync(nongoose, &user_friend_id)?;
  println!("User friend: {:?}", data);

  Ok(())
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "async", tokio::main)]
async fn main() -> nongoose::errors::Result<()> {
  let user_friend_id = ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap();
  let nongoose = get_instance();

  let data = run_async(nongoose, &user_friend_id).await?;
  println!("User friend: {:?}", data);

  Ok(())
}
