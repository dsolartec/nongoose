use mongodb::{
  bson::{doc, oid::ObjectId},
  sync::Client,
};
use nongoose::{schema_relations, Nongoose, Schema};
use serde::{Deserialize, Serialize};

#[schema_relations]
#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
struct Author {
  #[schema(id, unique)]
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[schema(unique)]
  pub username: String,

  #[schema(one_to_many = "Post")]
  #[serde(default, skip_serializing)]
  pub posts: Vec<Post>,
}

#[schema_relations]
#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
struct Post {
  #[schema(id, unique)]
  #[serde(rename = "_id")]
  pub id: ObjectId,

  pub title: String,

  #[schema(many_to_one = "Author", optional)]
  #[serde(skip_serializing)]
  pub author: Option<Author>,
}

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
    .add_schema::<Author>()
    .add_schema::<Post>()
    .add_schema::<Post>()
    .finish()
}

#[cfg(not(feature = "async"))]
fn main() {
  println!("Not implemented");
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "async", tokio::main)]
async fn main() -> nongoose::errors::Result<()> {
  let nongoose = get_instance();

  if let Some(author) = nongoose
    .find_one::<Author>(doc! { "username": "nongoose" })
    .await?
  {
    // Get author posts.
    let author = author.populate("posts").await?;

    println!("Author posts: {:?}", author.posts);
  } else {
    let author = Author {
      id: ObjectId::new(),
      username: String::from("nongoose"),
      posts: Vec::new(),
    };

    nongoose.create(&author).await?;

    let post_one = Post {
      id: ObjectId::new(),
      title: String::from("Nongoose example 1"),
      author: None,
      author_id: None,
    };

    nongoose.create(&post_one).await?;

    let post_two = Post {
      id: ObjectId::new(),
      title: String::from("Nongoose example 2"),
      author: Some(author.clone()),
      author_id: Some(author.id),
    };

    nongoose.create(&post_two).await?;

    let post_three = Post {
      id: ObjectId::new(),
      title: String::from("Nongoose example 3"),
      author: Some(author.clone()),
      author_id: Some(author.id),
    };

    nongoose.create(&post_three).await?;

    let post_four = Post {
      id: ObjectId::new(),
      title: String::from("Nongoose example 4"),
      author: Some(author.clone()),
      author_id: Some(author.id),
    };

    nongoose.create(&post_four).await?;

    // Get author posts
    let author = author.populate("posts").await?;

    println!("Author posts: {:?}", author.posts);
  }

  Ok(())
}
