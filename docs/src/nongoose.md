# Nongoose

## `Nongoose::build()`

**Arguments**

- database `mongodb::sync::Database` database instance

**Returns**

- `NongooseBuilder`

Create a builder for building `Nongoose`. On the builder, call `.add_schema::<T: Schema>()`(optional) to registry a schema. Finally, call `.finish()` to create the instance of `Nongoose`.

**Example**

```rust,no_run
let nongoose = Nongoose::build()
  .add_schema::<User>()
  .finish();
```

## `Nongoose.create()`

**Generics**

- T `Debug + Schema` value of schema to query by

**Arguments**

- data `&T` Document to insert

**Returns**

- `nongoose::errors::Result<T>`

Shortcut for saving one document to the database. `Nongoose.create(doc)` does `doc.save()`.

This function triggers `save()`.

**Example**

```rust,no_run
// Insert one new `User` document (Sync method)
match nongoose.create::<User>(&user) {
  Ok(user) => {
    println!("User saved: {}", user.id);
  },
  Err(error) => {
    eprintln!("Error saving the user: {}", error);
  }
}

// Insert one new `User` document (Async method)
match nongoose.create::<User>(&user).await {
  Ok(user) => {
    println!("User saved: {}", user.id);
  },
  Err(error) => {
    eprintln!("Error saving the user: {}", error);
  }
}
```

## `Nongoose.find_by_id()`

**Generics**

- T `Debug + Schema` value of schema to query by

**Arguments**

- id `&T::Id` value of `_id` to query by

**Returns**

- `nongoose::errors::Result<Option<T>>`

Finds a single document by its `_id` field. `find_by_id(id)`is almost equivalent to `find_one(doc! { "_id": id })`.
If you want to query by a document's `_id`, use `find_by_id()`instead of `find_one()`.

This function triggers `find_one()`.

**Example**

```rust,no_run
// Find one `User` document by `_id` (Sync method)
match nongoose.find_by_id::<User>(
  &ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap()
) {
  Ok(Some(user)) => println!("User found: {}", user.id),
  Ok(None) => eprintln!("Cannot find the user"),
  Err(error) => eprintln!("Error finding user: {}", error),
}

// Find one `User` document by `_id` (Async method)
match nongoose.find_by_id::<User>(
  &ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap()
).await {
  Ok(Some(user)) => println!("User found: {}", user.id),
  Ok(None) => eprintln!("Cannot find the user"),
  Err(error) => eprintln!("Error finding user: {}", error),
}
```

## `Nongoose.find_one()`

**Generics**

- T `Debug + Schema` value of schema to query by

**Arguments**

- conditions `bson::Document`

Finds one document.

**Returns**

- `nongoose::errors::Result<Option<T>>`

**Example**

```rust,no_run
// Find one user whose `username` is `nongoose` (Sync method)
match nongoose.find_one::<User>(doc! { "username": "nongoose" }) {
  Ok(Some(user)) => println!("User found: {}", user.id),
  Ok(None) => eprintln!("Cannot find the user"),
  Err(error) => eprintln!("Error finding user: {}", error),
}

// Find one user whose `username` is `nongoose` (Async method)
match nongoose.find_one::<User>(doc! { "username": "nongoose" }).await {
  Ok(Some(user)) => println!("User found: {}", user.id),
  Ok(None) => eprintln!("Cannot find the user"),
  Err(error) => eprintln!("Error finding user: {}", error),
}
```
