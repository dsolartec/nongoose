use nongoose::{
  schema_relations, bson::oid::ObjectId,
  Client, Nongoose, Schema, SchemaBefore,
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

impl User {
  pub fn new(username: &str) -> Self {
    Self {
      id: ObjectId::new(),
      username: String::from(username),
    }
  }
}

#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
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

#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
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

#[cfg(feature = "sync")]
fn run_sync(nongoose: Nongoose, user_friend_id: &ObjectId) -> nongoose::Result<UserFriend> {
  let user_friends = nongoose.find_by_id::<UserFriend>(user_friend_id);

  match user_friends {
    Ok(None) | Err(_) => {
      let user_one = User::new("nongoose").save()?;
      let user_two = User::new("nongoose2").save()?;

      let user_friend = UserFriend {
        id: *user_friend_id,
        from: Some(user_one.clone()),
        from_id: user_one.id,
        to: Some(user_two.clone()),
        to_id: user_two.id,
      }
      .save()?;

      Ok(user_friend)
    }
    Ok(Some(user_friends)) => Ok(user_friends.populate("from")?.populate("to")?),
  }
}

#[cfg(feature = "tokio-runtime")]
async fn run_async(
  nongoose: Nongoose,
  user_friend_id: &ObjectId,
) -> nongoose::Result<UserFriend> {
  let user_friends = nongoose.find_by_id::<UserFriend>(user_friend_id).await;

  match user_friends {
    Ok(None) | Err(_) => {
      let user_one = User::new("nongoose").save().await?;
      let user_two = User::new("nongoose2").save().await?;

      let user_friend = UserFriend {
        id: *user_friend_id,
        from: Some(user_one.clone()),
        from_id: user_one.id,
        to: Some(user_two.clone()),
        to_id: user_two.id,
      }
      .save()
      .await?;

      Ok(user_friend)
    }
    Ok(Some(user_friends)) => Ok(user_friends.populate("from").await?.populate("to").await?),
  }
}

#[cfg(feature = "sync")]
fn main() -> nongoose::Result<()> {
  let user_friend_id = ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap();
  let nongoose = get_instance();

  let data = run_sync(nongoose, &user_friend_id)?;
  println!("User friend: {:?}", data);

  Ok(())
}

#[cfg(feature = "tokio-runtime")]
#[cfg_attr(feature = "tokio-runtime", tokio::main)]
async fn main() -> nongoose::Result<()> {
  let user_friend_id = ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap();
  let nongoose = get_instance();

  let data = run_async(nongoose, &user_friend_id).await?;
  println!("User friend: {:?}", data);

  Ok(())
}
