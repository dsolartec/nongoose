# Nongoose

![Crates.io version](https://img.shields.io/crates/v/nongoose?label=version) ![Crates.io downloads](https://img.shields.io/crates/d/nongoose?label=downloads) ![License](https://img.shields.io/github/license/dsolartec/nongoose) ![GitHub repository stars](https://img.shields.io/github/stars/dsolartec/nongoose?style=social)

ODM for MongoDB based on Mongoose and written in Rust

## Basic usage

```rust
use nongoose::{bson::oid::ObjectId, Client, Schema};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
struct User {
  #[schema(id)]
  #[serde(rename = "_id")]
  pub id: ObjectId,

  #[schema(unique)]
  pub username: String,
}

#[tokio::main]
async fn main() {
  // Get MongoDB connection.
  let client = match Client::with_uri_str("mongodb://localhost:27017").await {
    Ok(client) => client,
    Err(e) => panic!("Error connecting to the database: {}", e),
  };

  // Nongoose instance.
  let nongoose = nongoose::Nongoose::build(client.database("nextchat"))
    .add_schema::<User>()
    .finish();

  let user = User {
    id: ObjectId::new(),
    username: String::from("nongoose"),
  };

  if let Err(error) = user.save().await {
    panic!("Cannot create the user: {}", error);
  }

  println!("User created in the database: {}", user.id);
}
```

## Tests

```sh
# Sync tests
$ DATABASE_URL=mongodb://localhost:27017 cargo test --no-default-features --features derive,sync

# Async tests (Tokio runtime)
$ DATABASE_URL=mongodb://localhost:27017 cargo test
```

## License

Check the [COPYING](./COPYING) file for more information.

## Related projects
- [wither](https://github.com/thedodd/wither)
- [MongODM](https://github.com/Devolutions/mongodm-rs)

## Contributors

Thanks to this amazing people for make Nongoose better:

- [@dsolartec](https://github.com/dsolartec)

> If you help to Nongoose feel free to add here.
