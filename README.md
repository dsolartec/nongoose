# Nongoose

MongoDB ODM for Rust based on Mongoose

## Basic usage

```rust
use mongodb::{bson::oid::ObjectId, Client};
use nongoose::Schema;
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
    Err(e) => {
      panic!("Error connecting to the database: {}", e);
    }
  };

  // Nongoose instance.
  let nongoose = nongoose::Nongoose::build(client.database("nextchat")).finish();

  let user = User {
    id: ObjectId::new(),
    username: String::from("nongoose"),
  };

  if let Err(error) = nongoose.create(&user).await {
    panic!("Cannot create the user: {}", error);
  }

  println!("User created in the database: {}", user.id);
}
```

## Attributes

```rust
#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
#[schema(name = "users")]   // <-- this is a container attribute
struct User {
  #[schema(id)] // <-- this is a field attribute
  #[serde(rename = "_id")]
  id: ObjectId;
}
```

### Container attributes

- `#[schema(name = "name")]`

  Set the collection name with the given name instead of its Rust name.

### Field attributes

- `#[schema(id)]` _Required_

  Represents the id of the document (`_id` in MongoDB).

- `#[schema(unique)]`

  Unique this field: the field value cannot be duplicated in the document.

- `#[schema(convert = "path")]`

  Call a function to convert the field type to a BSON type.

- `#[schema(many_to_one = "Schema")]`

  Many to one relation.

- `#[schema(one_to_one = "Schema")]`

  One to one relation.

## Examples

1. [Many to One relation](./examples/schema-relations.rs)

```sh
$ DATABASE_URL=mongodb://localhost:27017 cargo run --example many-to-one
```

## License

Check the [COPYING](./COPYING) file for more information.

## Contributors

Thanks to this amazing people for make Nongoose better:

- [@danielsolartech](https://github.com/danielsolartech) `NextChat Founder`

> If you help to Nongoose feel free to add here.
