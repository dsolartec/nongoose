# Schema

## `Schema.populate()`

**Arguments**

- field `&str` the field to populate

**Returns**

- `nongoose::errors::Result<Self>`

Populates fields on an existing schema.

**Example**

```rust,no_run
// Populate the role of the user (Sync method)
match user.clone().populate("role") {
  Ok(u) => user = u,
  Err(error) => eprintln!("Error populating user: {}", error),
}

// Populate the role of the user (Async method)
match user.clone().populate("role").await {
  Ok(u) => user = u,
  Err(error) => eprintln!("Error populating user: {}", error),
}
```

## `Schema.save()`

**Returns**

- `nongoose::errors::Result<Self>`

Saves this document by inserting a new document into the database if it does not exist before, or sends an `replace_one` operation with the modifications to the database.

If the document needs to be inserted to the database, the `SchemaBefore.before_create()` method is called before insert the document; otherwise, `SchemaBefore.before_update()` is called before replace the document.

**Example**

```rust,no_run
user.username = String::from("Nongoose");

// Sync method
match user.save() {
  Ok(u) => user = u,
  Err(error) => eprintln!("Error saving user: {}", error),
}

// Async method
match user.save().await {
  Ok(u) => user = u,
  Err(error) => eprintln!("Error saving user: {}", error),
}
```
