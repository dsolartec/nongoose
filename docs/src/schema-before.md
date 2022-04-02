# SchemaBefore Trait

## `SchemaBefore.before_create()`

**Arguments**

- db `&nongoose::mongodb::sync::Database` the schema database instance

**Returns**

- `nongoose::Result<()>`

Executes a custom validation before insert the document to the database.

**Example**

```rust,no_run
// Sync method
impl SchemaBefore for User {
  fn before_create(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }
}

// Async method
#[async_trait::async_trait]
impl SchemaBefore for User {
  async fn before_create(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }
}
```

## `SchemaBefore.before_delete()`

**Arguments**

- db `&nongoose::mongodb::sync::Database` the schema database instance

**Returns**

- `nongoose::Result<bool>` 

Executes a custom validation before delete the document from the database.

**Example**

```rust,no_run
// Sync method
impl SchemaBefore for User {
  fn before_delete(&mut self, _db: &Database) -> Result<bool> {
    Ok(true)
  }
}

// Async method
#[async_trait::async_trait]
impl SchemaBefore for User {
  async fn before_delete(&mut self, _db: &Database) -> Result<bool> {
    Ok(true)
  }
}
```

## `SchemaBefore.before_update()`

**Arguments**

- db `&nongoose::mongodb::sync::Database` the schema database instance

**Returns**

- `nongoose::Result<()>`

Executes a custom validation before replace the document in the database (called on `Schema.save()`).

**Example**

```rust,no_run
// Sync method
impl SchemaBefore for User {
  fn before_update(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }
}

// Async method
#[async_trait::async_trait]
impl SchemaBefore for User {
  async fn before_update(&mut self, _db: &Database) -> Result<()> {
    Ok(())
  }
}
```
