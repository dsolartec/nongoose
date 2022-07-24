use nongoose::{
	bson::{doc, oid::ObjectId, Document, Regex},
	schema_relations, Client, Nongoose, Schema, SchemaBefore,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
struct User {
	#[schema(id, unique)]
	#[serde(rename = "_id")]
	id: ObjectId,

	#[schema(unique)]
	username: String,

	realname: String,
	age: u64,
}

impl User {
	pub fn new(username: &str, realname: &str, age: u64) -> Self {
		Self {
			id: ObjectId::new(),
			username: String::from(username),
			realname: String::from(realname),
			age,
		}
	}
}

#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
impl SchemaBefore for User {}

#[schema_relations]
#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
struct Post {
  #[schema(id, unique)]
  #[serde(rename = "_id")]
  id: ObjectId,

  #[schema(many_to_one = "User")]
  #[serde(skip_serializing)]
  author: Option<User>,

  title: String,
}

impl Post {
	pub fn new(title: &str, author: &User) -> Self {
		Self {
			id: ObjectId::new(),
			author: Some(author.clone()),
			author_id: author.id,
			title: String::from(title),
		}
	}
}

#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
impl SchemaBefore for Post {}

#[schema_relations]
#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
struct PostComment {
	#[schema(id, unique)]
	#[serde(rename = "_id")]
	id: ObjectId,

	#[schema(many_to_one = "User")]
	#[serde(skip_serializing)]
	author: Option<User>,

	#[schema(many_to_one = "Post")]
	#[serde(skip_serializing)]
	post: Option<Post>,

	message: String,
}
  
impl PostComment {
	pub fn new(message: &str, post: &Post, author: &User) -> Self {
		Self {
			id: ObjectId::new(),
			author: Some(author.clone()),
			author_id: author.id,
			post: Some(post.clone()),
			post_id: post.id,
			message: String::from(message),
		}
	}
}

#[cfg_attr(feature = "tokio-runtime", async_trait::async_trait)]
impl SchemaBefore for PostComment {}

#[derive(Debug)]
struct SearchResult {
	posts_with_comments: Vec<String>,
	users_with_comments: Vec<String>,
}

impl From<Document> for SearchResult {
	fn from(document: Document) -> Self {
		Self {
			posts_with_comments: document
				.get_array("posts_with_comments")
				.unwrap_or(&Vec::new())
				.iter()
				.map(|d| d.as_str())
				.filter(|d| d.is_some())
				.map(|d| String::from(d.unwrap()))
				.collect(),
			users_with_comments: document
				.get_array("users_with_comments")
				.unwrap_or(&Vec::new())
				.iter()
				.map(|d| d.as_str())
				.filter(|d| d.is_some())
				.map(|d| String::from(d.unwrap()))
				.collect(),
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
		.add_schema::<User>()
		.add_schema::<Post>()
		.add_schema::<PostComment>()
		.build()
}

#[cfg(feature = "tokio-runtime")]
#[cfg_attr(feature = "tokio-runtime", tokio::test)]
async fn aggregate() {
	let nongoose = get_instance();

	// Create default users.
	let daniel = User::new("daniel", "Daniel Solarte", 19).save().await;
	assert!(daniel.is_ok());

	let robert = User::new("robert", "Robert", 25).save().await;
	assert!(robert.is_ok());

	let daniel = daniel.unwrap();
	let robert = robert.unwrap();

	// Create default posts.
	let nongoose_released = Post::new("Nongoose v0.1.0 released!", &robert).save().await;
	assert!(nongoose_released.is_ok());

	let nongoose_released = nongoose_released.unwrap();

	// Create default post comments.
	let comment1 = PostComment::new("Hello! First comment", &nongoose_released, &daniel)
		.save()
		.await;
	assert!(comment1.is_ok());

	let comment2 = PostComment::new("OMG, aggregations finally! :)", &nongoose_released, &robert)
		.save()
		.await;
	assert!(comment2.is_ok());

	// Aggregation
	let result = nongoose
		.aggregate::<PostComment, SearchResult>(
			vec![
				doc! {
					"$match": {
						"message": Regex { pattern: String::from(" "), options: String::new() },
					},
				},
				doc! {
					"$lookup": {
						"from": User::collection_name(),
						"localField": "author_id",
						"foreignField": "_id",
						"as": "users",
					}
				},
				doc! {
					"$lookup": {
						"from": Post::collection_name(),
						"localField": "post_id",
						"foreignField": "_id",
						"as": "posts",
					},
				},
				doc! {
					"$group": {
						"_id": "1",
						"posts_with_comments": { "$addToSet": { "$first": "$posts.title" } },
						"users_with_comments": { "$addToSet": { "$first": "$users.realname" } },
					},
				},
			],
			None,
		)
		.await;
	assert!(result.is_ok());

	let result = result.unwrap();
	assert_eq!(result.len(), 1);

	let result_0 = result.get(0).unwrap();
	assert_eq!(result_0.posts_with_comments.len(), 1);
	assert_eq!(result_0.users_with_comments.len(), 2);
}
  
#[cfg(feature = "sync")]
#[cfg_attr(feature = "sync", test)]
fn aggregate() {
	let nongoose = get_instance();

	// Create default users.
	let daniel = User::new("daniel", "Daniel Solarte", 19).save();
	assert!(daniel.is_ok());

	let robert = User::new("robert", "Robert", 25).save();
	assert!(robert.is_ok());

	let daniel = daniel.unwrap();
	let robert = robert.unwrap();

	// Create default posts.
	let nongoose_released = Post::new("Nongoose v0.1.0 released!", &robert).save();
	assert!(nongoose_released.is_ok());

	let nongoose_released = nongoose_released.unwrap();

	// Create default post comments.
	let comment1 = PostComment::new("Hello! First comment", &nongoose_released, &daniel).save();
	assert!(comment1.is_ok());

	let comment2 = PostComment::new("OMG, aggregations finally! :)", &nongoose_released, &robert).save();
	assert!(comment2.is_ok());

	// Aggregation
	let result = nongoose
	  	.aggregate::<PostComment, SearchResult>(
			vec![
				doc! {
					"$match": {
						"message": Regex { pattern: String::from(" "), options: String::new() },
					},
				},
				doc! {
					"$lookup": {
						"from": User::collection_name(),
						"localField": "author_id",
						"foreignField": "_id",
						"as": "users",
					}
				},
				doc! {
					"$lookup": {
						"from": Post::collection_name(),
						"localField": "post_id",
						"foreignField": "_id",
						"as": "posts",
					},
				},
				doc! {
					"$group": {
						"_id": "1",
						"posts_with_comments": { "$addToSet": { "$first": "$posts.title" } },
						"users_with_comments": { "$addToSet": { "$first": "$users.realname" } },
					},
				},
			],
			None,
	  	);
	assert!(result.is_ok());

	let result = result.unwrap();
	assert_eq!(result.len(), 1);

	let result_0 = result.get(0).unwrap();
	assert_eq!(result_0.posts_with_comments.len(), 1);
	assert_eq!(result_0.users_with_comments.len(), 2);
}
