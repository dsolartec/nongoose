use nongoose::{
  schema_relations, bson::{doc, oid::ObjectId}, Client, Nongoose, Schema, SchemaBefore
};
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

impl Author {
  pub fn new(username: &str) -> Self {
    Self {
      id: ObjectId::new(),
      username: String::from(username),
      posts: Vec::new(),
    }
  }
}

#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
impl SchemaBefore for Author {}

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

impl Post {
  pub fn new(title: &str) -> Self {
    Self {
      id: ObjectId::new(),
      title: String::from(title),
      author_id: None,
      author: None,
    }
  }

  pub fn new_with_author(title: &str, author: &Author) -> Self {
    Self {
      id: ObjectId::new(),
      title: String::from(title),
      author: Some(author.clone()),
      author_id: Some(author.id),
    }
  }
}

#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
impl SchemaBefore for Post {}

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
    .finish()
}

#[cfg(feature = "sync")]
fn main() -> nongoose::Result<()> {
  let nongoose = get_instance();

  if let Some(author) = nongoose.find_one::<Author>(doc! { "username": "nongoose" }, None)? {
    // Get author posts.
    let author = author.populate("posts")?;
    println!("Author posts: {:?}", author.posts);
  } else {
    // Authors
    let author = Author::new("nongoose").save()?;

    // Posts
    Post::new("Nongoose example 1").save()?;
    Post::new_with_author("Nongoose example 2", &author).save()?;
    Post::new_with_author("Nongoose example 3", &author).save()?;
    Post::new_with_author("Nongoose example 4", &author).save()?;

    // Get author posts
    let author = author.populate("posts")?;
    println!("Author posts: {:?}", author.posts);
  }

  Ok(())
}

#[cfg(feature = "tokio-runtime")]
#[cfg_attr(feature = "tokio-runtime", tokio::main)]
async fn main() -> nongoose::Result<()> {
  let nongoose = get_instance();

  if let Some(author) = nongoose
    .find_one::<Author>(doc! { "username": "nongoose" }, None)
    .await?
  {
    // Get author posts.
    let author = author.populate("posts").await?;
    println!("Author posts: {:?}", author.posts);
  } else {
    // Authors
    let author = Author::new("nongoose").save().await?;

    // Posts
    Post::new("Nongoose example 1").save().await?;
    Post::new_with_author("Nongoose example 2", &author)
      .save()
      .await?;
    Post::new_with_author("Nongoose example 3", &author)
      .save()
      .await?;
    Post::new_with_author("Nongoose example 4", &author)
      .save()
      .await?;

    // Get author posts
    let author = author.populate("posts").await?;
    println!("Author posts: {:?}", author.posts);
  }

  Ok(())
}
