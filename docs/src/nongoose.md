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

## `Nongoose.find()`

**Generics**

- T `Debug + Schema` value of schema to query by

**Arguments**

- conditions `bson::Document`
- options `mongodb::options::FindOptions`

**Returns**

- `nongoose::errors::Result<Vec<T>>`

Finds documents.

**Example**

```rust,no_run
// Sync method
match nongoose.find::<User>(
  doc! { "age": { "$gte": 18 } },
  Some(Findptions::builder().sort(doc! { "username": 1 }).build())
) {
  Ok(users) => println!("Found {} users!", users.len()),
  Err(error) => eprintln!("Error finding users: {}", error),
}

// Async method
match nongoose.find::<User>(
  doc! { "age": { "$gte": 18 } },
  Some(FindOptions::builder().sort(doc! { "username": 1 }).build())
).await {
  Ok(users) => println!("Found {} users!", users.len()),
  Err(error) => eprintln!("Error finding users: {}", error),
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
- options `mongodb::options::FindOneOptions`

**Returns**

- `nongoose::errors::Result<Option<T>>`

Finds one document.

**Example**

```rust,no_run
// Find one user whose `username` is `nongoose` (Sync method)
match nongoose.find_one::<User>(doc! { "username": "nongoose" }, None) {
  Ok(Some(user)) => println!("User found: {}", user.id),
  Ok(None) => eprintln!("Cannot find the user"),
  Err(error) => eprintln!("Error finding user: {}", error),
}

// Find one user whose `username` is `nongoose` (Async method)
match nongoose.find_one::<User>(doc! { "username": "nongoose" }, None).await {
  Ok(Some(user)) => println!("User found: {}", user.id),
  Ok(None) => eprintln!("Cannot find the user"),
  Err(error) => eprintln!("Error finding user: {}", error),
}

// Passing options
match nongoose.find_one::<User>(
  doc! { "age": { "$gte": 18 } },
  Some(FindOneOptions::builder().sort(doc! { "username": 1 }).build())
) {
  Ok(Some(user)) => println!("User found: {}", user.id),
  Ok(None) => eprintln!("No users over 18 years old"),
  Err(error) => eprintln!("Error finding user: {}", error),
}
```

## `Nongoose.update_many()`

**Generics**

- T `Debug + Schema` value of schema to query by

**Arguments**

- conditions `bson::Document`
- data `bson::Document`
- options `mongodb::options::UpdateOptions`

**Returns**

- `nongoose::errors::Result<mongodb::results::UpdateResult>`

Updates _all_ documents in the database that match `conditions` without returning them.

**Note** update_many will _not_ fire update middleware (`SchemaBefore::before_update()`).

**Example**
```rust,no_run
// Update the age to 18 if it is under 18 (Sync method)
match nongoose.update_many::<User>(
  doc! { "age": { "$lt": 18 } },
  doc! { "$set": { "age": 18 } },
  None
) {
  Ok(result) => println!("Modified {} documents", result.modified_count),
  Err(error) => eprintln!("Error updating users: {}", error),
}

// Update the age to 18 if it is under 18 (Async method)
match nongoose.update_many::<User>(
  doc! { "age": { "$lt": 18 } },
  doc! { "$set": { "age": 18 } },
  None
).await {
  Ok(result) => println!("Modified {} documents", result.modified_count),
  Err(error) => eprintln!("Error updating users: {}", error),
}
```
