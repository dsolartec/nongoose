use mongodb::{
  bson::{doc, oid::ObjectId},
  sync::Client,
};
use nongoose::{Nongoose, Schema, SchemaBefore};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Schema, Serialize)]
struct Article {
  #[schema(id, unique)]
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub content: String,
}

impl Article {
  pub fn new(title: &str, content: &str) -> Self {
    Self {
      id: ObjectId::new(),
      title: String::from(title),
      content: String::from(content),
    }
  }
}

impl SchemaBefore for Article {}

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
    .add_schema::<Article>()
    .finish()
}

#[cfg(not(feature = "async"))]
#[test]
fn update_many() {
  let nongoose = get_instance();

  // Upload data
  let mut article_one = Article::new("Title 1", "Content 1");

  let article_one_saved = article_one.save();
  assert!(article_one_saved.is_ok());

  let article_one_saved = article_one_saved.unwrap();
  assert_eq!(article_one, article_one_saved);

  let mut article_two = Article::new("Title 2", "Content 2");

  let article_two_saved = article_two.save();
  assert!(article_two_saved.is_ok());

  let article_two_saved = article_two_saved.unwrap();
  assert_eq!(article_two, article_two_saved);

  // Update the content of two articles
  let result = nongoose.update_many::<Article>(
    doc! { "_id": { "$in": [article_one.id, article_two.id] } },
    doc! { "$set": { "content": "Content 3" } },
    None,
  );
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.modified_count, 2);

  // Verify new data
  let new_article_one = nongoose.find_by_id::<Article>(&article_one.id);
  assert!(new_article_one.is_ok());

  let new_article_one = new_article_one.unwrap();
  assert!(new_article_one.is_some());

  let new_article_one = new_article_one.unwrap();
  assert_eq!(new_article_one.title, article_one.title);
  assert_eq!(new_article_one.content, String::from("Content 3"));

  let new_article_two = nongoose.find_by_id::<Article>(&article_two.id);
  assert!(new_article_two.is_ok());

  let new_article_two = new_article_two.unwrap();
  assert!(new_article_two.is_some());

  let new_article_two = new_article_two.unwrap();
  assert_eq!(new_article_two.title, article_two.title);
  assert_eq!(new_article_two.content, String::from("Content 3"));

  assert_eq!(new_article_two.content, new_article_one.content);
}

#[cfg(feature = "async")]
#[cfg_attr(feature = "async", tokio::test)]
async fn update_many() {
  let nongoose = get_instance();

  // Upload data
  let mut article_one = Article::new("Title 1", "Content 1");

  let article_one_saved = article_one.save().await;
  assert!(article_one_saved.is_ok());

  let article_one_saved = article_one_saved.unwrap();
  assert_eq!(article_one, article_one_saved);

  let mut article_two = Article::new("Title 2", "Content 2");

  let article_two_saved = article_two.save().await;
  assert!(article_two_saved.is_ok());

  let article_two_saved = article_two_saved.unwrap();
  assert_eq!(article_two, article_two_saved);

  // Update the content of two articles
  let result = nongoose
    .update_many::<Article>(
      doc! { "_id": { "$in": [article_one.id, article_two.id] } },
      doc! { "$set": { "content": "Content 3" } },
      None,
    )
    .await;
  assert!(result.is_ok());

  let result = result.unwrap();
  assert_eq!(result.modified_count, 2);

  // Verify new data
  let new_article_one = nongoose.find_by_id::<Article>(&article_one.id).await;
  assert!(new_article_one.is_ok());

  let new_article_one = new_article_one.unwrap();
  assert!(new_article_one.is_some());

  let new_article_one = new_article_one.unwrap();
  assert_eq!(new_article_one.title, article_one.title);
  assert_eq!(new_article_one.content, String::from("Content 3"));

  let new_article_two = nongoose.find_by_id::<Article>(&article_two.id).await;
  assert!(new_article_two.is_ok());

  let new_article_two = new_article_two.unwrap();
  assert!(new_article_two.is_some());

  let new_article_two = new_article_two.unwrap();
  assert_eq!(new_article_two.title, article_two.title);
  assert_eq!(new_article_two.content, String::from("Content 3"));

  assert_eq!(new_article_two.content, new_article_one.content);
}
