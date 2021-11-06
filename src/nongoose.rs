mod builder;
pub(crate) mod globals;

pub use builder::NongooseBuilder;
use mongodb::{
  bson::{doc, Document},
  options::{FindOneOptions, FindOptions, UpdateOptions},
  results::UpdateResult,
  sync::Database,
};
#[cfg(feature = "async")]
use tokio::task::spawn_blocking;

use crate::{errors::Result, Schema};

#[derive(Clone)]
pub struct Nongoose {
  builder: NongooseBuilder,
}

impl Nongoose {
  pub fn build(database: Database) -> NongooseBuilder {
    NongooseBuilder {
      database,
      schemas: Vec::new(),
    }
  }

  /// Shortcut for saving one document to the database. `Nongoose.create(doc)` does `Schema.save()`.
  ///
  /// This function triggers `Schema.save()`.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Insert one new `User` document
  /// match nongoose.create::<User>(&user) {
  ///   Ok(user) => println!("User saved: {}", user.id),
  ///   Err(error) => eprintln!("Error saving the user: {}", error),
  /// }
  /// ```
  #[cfg(not(feature = "async"))]
  pub fn create<T>(&self, data: &T) -> Result<T>
  where
    T: Schema + Clone,
  {
    data.clone().save()
  }

  /// Shortcut for saving one document to the database. `Nongoose.create(doc)` does `Schema.save()`.
  ///
  /// This function triggers `Schema.save()`.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Insert one new `User` document
  /// match nongoose.create::<User>(&user).await {
  ///   Ok(user) => println!("User saved: {}", user.id);,
  ///   Err(error) => eprintln!("Error saving the user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "async")]
  pub async fn create<T>(&self, data: &T) -> Result<T>
  where
    T: Schema + Clone + Send + 'static,
  {
    data.clone().save().await
  }

  /// Finds documents.
  ///
  /// # Options
  /// ```rust,no_run,ignore
  /// FindOptions::builder()
  ///   // Optional (bool)
  ///   // Enables writing to temporary files by the server. When set to true, the find operation can write data to the _tmp subdirectory in the dbPath directory.
  ///   // Only supported in server versions 4.4+.
  ///   .allow_disk_use(...)
  ///   // Optional (bool)
  ///   // If true, partial results will be returned from a mongos rather than an error being returned if one or more shards is down.
  ///   .allow_partial_results(...)
  ///   // Optional (u32)
  ///   // The number of documents the server should return per cursor batch.
  ///   // Note that this does not have any affect on the documents that are returned by a cursor, only the number of documents kept in memory at a given time
  ///   // (and by extension, the number of round trips needed to return the entire set of documents returned by the query.
  ///   .batch_size(...)
  ///   // Optional (String)
  ///   // Tags the query with an arbitrary string to help trace the operation through the database profiler, currentOp and logs.
  ///   .comment(...)
  ///   // Optional (mongodb::options::CursorType)
  ///   // The type of cursor to return.
  ///   .cursor_type(...)
  ///   // Optional (mongodb::options::Hint)
  ///   // The index to use for the operation.
  ///   .hint(...)
  ///   // Optional (i64)
  ///   // The maximum number of documents to query. If a negative number is specified, the documents will be returned in a single batch limited in number
  ///   // by the positive value of the specified limit.
  ///   .limit(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The exclusive upper bound for a specific index.
  ///   .max(...)
  ///   // Optional (std::time::Duration)
  ///   // The maximum amount of time for the server to wait on new documents to satisfy a tailable cursor query. If the cursor is not tailable, this option is ignored.
  ///   .max_await_time(...)
  ///   // Optional (u64)
  ///   // Maximum number of documents or index keys to scan when executing the query.
  ///   // Note: this option is deprecated starting in MongoDB version 4.0 and removed in MongoDB 4.2. Use the maxTimeMS option instead.
  ///   .max_scan(...)
  ///   // Optional (std::time::Duration)
  ///   // The maximum amount of time to allow the query to run.
  ///   // This options maps to the `maxTimeMS` MongoDB query option, so the duration will be sent across the wire as an integer number of milliseconds.
  ///   .max_time(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The inclusive lower bound for a specific index.
  ///   .min(...)
  ///   // Optional (bool)
  ///   // Whether the server should close the cursor after a period of inactivity.
  ///   .no_cursor_timeout(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // Limits the fields of the document being returned.
  ///   .projection(...)
  ///   // Optional (mongodb::options::ReadConcern)
  ///   // The read concern to use for this find query.
  ///   // If none specified, the default set on the collection will be used.
  ///   .read_concern(...)
  ///   // Optional (bool)
  ///   // Whether to return only the index keys in the documents.
  ///   .return_key(...)
  ///   // Optional (mongodb::options::SelectionCriteria)
  ///   // The criteria used to select a server for this find query.
  ///   // If none specified, the default set on the collection will be used.
  ///   .selection_criteria(...)
  ///   // Optional (bool)
  ///   // Whether to return the record identifier for each document.
  ///   .show_record_id(...)
  ///   // Optional (u64)
  ///   // The number of documents to skip before counting.
  ///   .skip(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The order of the documents for the purposes of the operation.
  ///   .sort(...)
  ///   // Optional (mongodb::options::Collation)
  ///   // The collation to use for the operation.
  ///   // See the [documentation](https://docs.mongodb.com/manual/reference/collation/) for more information on how to use this option.
  ///   .collation(...)
  ///   // Required to create the instance of `FindOptions`
  ///   .build()
  /// ```
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Search for users over 18 years of age
  /// match nongoose.find::<User>(doc! { "age": { "$gte": 18 } }, None) {
  ///   Ok(users) => println!("Found {} users!", users.len()),
  ///   Err(error) => eprintln!("Error finding users: {}", error),
  /// }
  ///
  /// // Passing arguments
  /// match nongoose.find::<User>(
  ///   doc! { "age": { "$gte": 18 } },
  ///   Some(FindOptions::builder().sort(doc! { "username": 1 }).build())
  /// ) {
  ///   Ok(users) => println!("Found {} users!", users.len()),
  ///   Err(error) => eprintln!("Error finding users: {}", error),
  /// }
  /// ```
  #[cfg(not(feature = "async"))]
  pub fn find<T>(&self, conditions: Document, options: Option<FindOptions>) -> Result<Vec<T>>
  where
    T: core::fmt::Debug + Schema,
  {
    self.builder.find_sync(conditions, options)
  }

  /// Finds documents.
  ///
  /// # Options
  /// ```rust,no_run,ignore
  /// FindOptions::builder()
  ///   // Optional (bool)
  ///   // Enables writing to temporary files by the server. When set to true, the find operation can write data to the _tmp subdirectory in the dbPath directory.
  ///   // Only supported in server versions 4.4+.
  ///   .allow_disk_use(...)
  ///   // Optional (bool)
  ///   // If true, partial results will be returned from a mongos rather than an error being returned if one or more shards is down.
  ///   .allow_partial_results(...)
  ///   // Optional (u32)
  ///   // The number of documents the server should return per cursor batch.
  ///   // Note that this does not have any affect on the documents that are returned by a cursor, only the number of documents kept in memory at a given time
  ///   // (and by extension, the number of round trips needed to return the entire set of documents returned by the query.
  ///   .batch_size(...)
  ///   // Optional (String)
  ///   // Tags the query with an arbitrary string to help trace the operation through the database profiler, currentOp and logs.
  ///   .comment(...)
  ///   // Optional (mongodb::options::CursorType)
  ///   // The type of cursor to return.
  ///   .cursor_type(...)
  ///   // Optional (mongodb::options::Hint)
  ///   // The index to use for the operation.
  ///   .hint(...)
  ///   // Optional (i64)
  ///   // The maximum number of documents to query. If a negative number is specified, the documents will be returned in a single batch limited in number
  ///   // by the positive value of the specified limit.
  ///   .limit(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The exclusive upper bound for a specific index.
  ///   .max(...)
  ///   // Optional (std::time::Duration)
  ///   // The maximum amount of time for the server to wait on new documents to satisfy a tailable cursor query. If the cursor is not tailable, this option is ignored.
  ///   .max_await_time(...)
  ///   // Optional (u64)
  ///   // Maximum number of documents or index keys to scan when executing the query.
  ///   // Note: this option is deprecated starting in MongoDB version 4.0 and removed in MongoDB 4.2. Use the maxTimeMS option instead.
  ///   .max_scan(...)
  ///   // Optional (std::time::Duration)
  ///   // The maximum amount of time to allow the query to run.
  ///   // This options maps to the `maxTimeMS` MongoDB query option, so the duration will be sent across the wire as an integer number of milliseconds.
  ///   .max_time(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The inclusive lower bound for a specific index.
  ///   .min(...)
  ///   // Optional (bool)
  ///   // Whether the server should close the cursor after a period of inactivity.
  ///   .no_cursor_timeout(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // Limits the fields of the document being returned.
  ///   .projection(...)
  ///   // Optional (mongodb::options::ReadConcern)
  ///   // The read concern to use for this find query.
  ///   // If none specified, the default set on the collection will be used.
  ///   .read_concern(...)
  ///   // Optional (bool)
  ///   // Whether to return only the index keys in the documents.
  ///   .return_key(...)
  ///   // Optional (mongodb::options::SelectionCriteria)
  ///   // The criteria used to select a server for this find query.
  ///   // If none specified, the default set on the collection will be used.
  ///   .selection_criteria(...)
  ///   // Optional (bool)
  ///   // Whether to return the record identifier for each document.
  ///   .show_record_id(...)
  ///   // Optional (u64)
  ///   // The number of documents to skip before counting.
  ///   .skip(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The order of the documents for the purposes of the operation.
  ///   .sort(...)
  ///   // Optional (mongodb::options::Collation)
  ///   // The collation to use for the operation.
  ///   // See the [documentation](https://docs.mongodb.com/manual/reference/collation/) for more information on how to use this option.
  ///   .collation(...)
  ///   // Required to create the instance of `FindOptions`
  ///   .build()
  /// ```
  ///
  /// See more [here](https://docs.rs/mongodb/2.0.1/mongodb/options/struct.FindOptions.html)
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Search for users over 18 years of age
  /// match nongoose.find::<User>(doc! { "age": { "$gte": 18 } }, None).await {
  ///   Ok(users) => println!("Found {} users!", users.len()),
  ///   Err(error) => eprintln!("Error finding users: {}", error),
  /// }
  ///
  /// // Passing arguments
  /// match nongoose.find::<User>(
  ///   doc! { "age": { "$gte": 18 } },
  ///   Some(FindOptions::builder().sort(doc! { "username": 1 }).build())
  /// ).await {
  ///   Ok(users) => println!("Found {} users!", users.len()),
  ///   Err(error) => eprintln!("Error finding users: {}", error),
  /// }
  /// ```
  #[cfg(feature = "async")]
  pub async fn find<T>(&self, conditions: Document, options: Option<FindOptions>) -> Result<Vec<T>>
  where
    T: core::fmt::Debug + Schema + 'static,
  {
    let builder = self.builder.clone();
    spawn_blocking(move || builder.find_sync(conditions, options)).await?
  }

  /// Finds a single document by its `_id` field. `find_by_id(id)`is almost equivalent to `find_one(doc! { "_id": id })`.
  /// If you want to query by a document's `_id`, use `find_by_id()`instead of `find_one()`.
  ///
  /// This function triggers `find_one()`.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Find one `User` document by `_id`
  /// match nongoose.find_by_id::<User>(&ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap()) {
  ///   Ok(Some(user)) => println!("User found: {}", user.id),
  ///   Ok(None) => eprintln!("Cannot find the user"),
  ///   Err(error) => eprintln!("Error finding user: {}", error),
  /// }
  /// ```
  #[cfg(not(feature = "async"))]
  pub fn find_by_id<T>(&self, id: &T::Id) -> Result<Option<T>>
  where
    T: core::fmt::Debug + Schema,
  {
    self.find_one(doc! { "_id": id.clone().into() }, None)
  }

  /// Finds a single document by its `_id` field. `find_by_id(id)`is almost equivalent to `find_one(doc! { "_id": id })`.
  /// If you want to query by a document's `_id`, use `find_by_id()`instead of `find_one()`.
  ///
  /// This function triggers `find_one()`.
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Find one `User` document by `_id`
  /// match nongoose.find_by_id::<User>(&ObjectId::parse_str("616c91dc8cb70be8cc7d1f38").unwrap()).await {
  ///   Ok(Some(user)) => println!("User found: {}", user.id),
  ///   Ok(None) => eprintln!("Cannot find the user"),
  ///   Err(error) => eprintln!("Error finding user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "async")]
  pub async fn find_by_id<T>(&self, id: &T::Id) -> Result<Option<T>>
  where
    T: core::fmt::Debug + Schema + 'static,
  {
    self.find_one(doc! { "_id": id.clone().into() }, None).await
  }

  /// Finds one document.
  ///
  /// # Options
  /// ```rust,no_run,ignore
  /// FindOneOptions::builder()
  ///   // Optional (bool)
  ///   // If true, partial results will be returned from a mongos rather than an error being returned if one or more shards is down.
  ///   .allow_partial_results(...)
  ///   // Optional (mongodb::options::Collation)
  ///   // The collation to use for the operation.
  ///   // See the [documentation](https://docs.mongodb.com/manual/reference/collation/) for more information on how to use this option.
  ///   .collation(...)
  ///   // Optional (String)
  ///   // Tags the query with an arbitrary string to help trace the operation through the database profiler, currentOp and logs.
  ///   .comment(...)
  ///   // Optional (mongodb::options::Hint)
  ///   // The index to use for the operation.
  ///   .hint(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The exclusive upper bound for a specific index.
  ///   .max(...)
  ///   // Optional (u64)
  ///   // Maximum number of documents or index keys to scan when executing the query.
  ///   // Note: this option is deprecated starting in MongoDB version 4.0 and removed in MongoDB 4.2. Use the maxTimeMS option instead.
  ///   .max_scan(...)
  ///   // Optional (std::time::Duration)
  ///   // The maximum amount of time to allow the query to run.
  ///   // This options maps to the `maxTimeMS` MongoDB query option, so the duration will be sent across the wire as an integer number of milliseconds.
  ///   .max_time(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The inclusive lower bound for a specific index.
  ///   .min(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // Limits the fields of the document being returned.
  ///   .projection(...)
  ///   // Optional (mongodb::options::ReadConcern)
  ///   // The read concern to use for this find query.
  ///   // If none specified, the default set on the collection will be used.
  ///   .read_concern(...)
  ///   // Optional (bool)
  ///   // Whether to return only the index keys in the documents.
  ///   .return_key(...)
  ///   // Optional (mongodb::options::SelectionCriteria)
  ///   // The criteria used to select a server for this find query.
  ///   // If none specified, the default set on the collection will be used.
  ///   .selection_criteria(...)
  ///   // Optional (bool)
  ///   // Whether to return the record identifier for each document.
  ///   .show_record_id(...)
  ///   // Optional (u64)
  ///   // The number of documents to skip before counting.
  ///   .skip(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The order of the documents for the purposes of the operation.
  ///   .sort(...)
  ///   // Required to create the instance of `FindOneOptions`
  ///   .build()
  /// ```
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Find one user whose `username` is `nongoose`
  /// match nongoose.find_one::<User>(doc! { "username": "nongoose" }, None) {
  ///   Ok(Some(user)) => println!("User found: {}", user.id),
  ///   Ok(None) => eprintln!("Cannot find the user"),
  ///   Err(error) => eprintln!("Error finding user: {}", error),
  /// }
  ///
  /// // Passing options
  /// match nongoose.find_one::<User>(
  ///   doc! { "username": "nongoose" },
  ///   Some(FindOneOptions::builder().sort(doc! { "username": 1 }).build())
  /// ) {
  ///   Ok(Some(user)) => println!("User found: {}", user.id),
  ///   Ok(None) => eprintln!("Cannot find the user"),
  ///   Err(error) => eprintln!("Error finding user: {}", error),
  /// }
  /// ```
  #[cfg(not(feature = "async"))]
  pub fn find_one<T>(
    &self,
    conditions: Document,
    options: Option<FindOneOptions>,
  ) -> Result<Option<T>>
  where
    T: core::fmt::Debug + Schema,
  {
    self.builder.find_one_sync(conditions, options)
  }

  /// Finds one document.
  ///
  /// # Options
  /// ```rust,no_run,ignore
  /// FindOneOptions::builder()
  ///   // Optional (bool)
  ///   // If true, partial results will be returned from a mongos rather than an error being returned if one or more shards is down.
  ///   .allow_partial_results(...)
  ///   // Optional (mongodb::options::Collation)
  ///   // The collation to use for the operation.
  ///   // See the [documentation](https://docs.mongodb.com/manual/reference/collation/) for more information on how to use this option.
  ///   .collation(...)
  ///   // Optional (String)
  ///   // Tags the query with an arbitrary string to help trace the operation through the database profiler, currentOp and logs.
  ///   .comment(...)
  ///   // Optional (mongodb::options::Hint)
  ///   // The index to use for the operation.
  ///   .hint(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The exclusive upper bound for a specific index.
  ///   .max(...)
  ///   // Optional (u64)
  ///   // Maximum number of documents or index keys to scan when executing the query.
  ///   // Note: this option is deprecated starting in MongoDB version 4.0 and removed in MongoDB 4.2. Use the maxTimeMS option instead.
  ///   .max_scan(...)
  ///   // Optional (std::time::Duration)
  ///   // The maximum amount of time to allow the query to run.
  ///   // This options maps to the `maxTimeMS` MongoDB query option, so the duration will be sent across the wire as an integer number of milliseconds.
  ///   .max_time(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The inclusive lower bound for a specific index.
  ///   .min(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // Limits the fields of the document being returned.
  ///   .projection(...)
  ///   // Optional (mongodb::options::ReadConcern)
  ///   // The read concern to use for this find query.
  ///   // If none specified, the default set on the collection will be used.
  ///   .read_concern(...)
  ///   // Optional (bool)
  ///   // Whether to return only the index keys in the documents.
  ///   .return_key(...)
  ///   // Optional (mongodb::options::SelectionCriteria)
  ///   // The criteria used to select a server for this find query.
  ///   // If none specified, the default set on the collection will be used.
  ///   .selection_criteria(...)
  ///   // Optional (bool)
  ///   // Whether to return the record identifier for each document.
  ///   .show_record_id(...)
  ///   // Optional (u64)
  ///   // The number of documents to skip before counting.
  ///   .skip(...)
  ///   // Optional (mongodb::bson::Document)
  ///   // The order of the documents for the purposes of the operation.
  ///   .sort(...)
  ///   // Required to create the instance of `FindOneOptions`
  ///   .build()
  /// ```
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Find one user whose `username` is `nongoose`
  /// match nongoose.find_one::<User>(doc! { "username": "nongoose" }, None).await {
  ///   Ok(Some(user)) => println!("User found: {}", user.id),
  ///   Ok(None) => eprintln!("Cannot find the user"),
  ///   Err(error) => eprintln!("Error finding user: {}", error),
  /// }
  ///
  /// // Passing options
  /// match nongoose.find_one::<User>(
  ///   doc! { "username": "nongoose" },
  ///   Some(FindOneOptions::builder().sort(doc! { "username": 1 }).build())
  /// ).await {
  ///   Ok(Some(user)) => println!("User found: {}", user.id),
  ///   Ok(None) => eprintln!("Cannot find the user"),
  ///   Err(error) => eprintln!("Error finding user: {}", error),
  /// }
  /// ```
  #[cfg(feature = "async")]
  pub async fn find_one<T>(
    &self,
    conditions: Document,
    options: Option<FindOneOptions>,
  ) -> Result<Option<T>>
  where
    T: core::fmt::Debug + Schema + 'static,
  {
    let builder = self.builder.clone();
    spawn_blocking(move || builder.find_one_sync(conditions, options)).await?
  }

  /// Updates _all_ documents in the database that match `conditions` without returning them.
  ///
  /// **Note** update_many will _not_ fire update middleware (`SchemaBefore::before_update()`).
  ///
  /// # Options
  /// ```rust,no_run,ignore
  /// UpdateOptions::builder()
  ///   // Optional (Vec<mongodb::bson::Document>)
  ///   // A set of filters specifying to which array elements an update should apply.
  ///   // See the documentation [here](https://docs.mongodb.com/manual/reference/command/update/) for more information on array filters.
  ///   .array_filters(...)
  ///   // Optional (bool)
  ///   // Opt out of document-level validation.
  ///   .bypass_document_validation(...)
  ///   // Optional (bool)
  ///   // If true, insert a document if no matching document is found.
  ///   .upsert(...)
  ///   // Optional (mongodb::options::Collation)
  ///   // The collation to use for the operation.
  ///   .collation(...)
  ///   // Optional (mongodb::options::Hint)
  ///   // A document or string that specifies the index to use to support the query predicate.
  ///   // Only available in MongoDB 4.2+. See the official MongoDB [documentation](https://docs.mongodb.com/manual/reference/command/update/#ex-update-command-hint) for examples.
  ///   .hint(...)
  ///   // Optional (mongodb::options::WriteConcern)
  ///   // The write concern for the operation.
  ///   .write_concern(...)
  ///   // Required to create the instance of `UpdateOptions`
  ///   .build()
  /// ```
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Update the age to 18 if it is under 18
  /// match nongoose.update_many::<User>(
  ///   doc! { "age": { "$lt": 18 } },
  ///   doc! { "$set": { "age": 18 } },
  ///   None
  /// ) {
  ///   Ok(result) => println!("Modified {} documents", result.modified_count),
  ///   Err(error) => eprintln!("Error updating users: {}", error),
  /// }
  /// ```
  #[cfg(not(feature = "async"))]
  pub fn update_many<T>(
    &self,
    conditions: Document,
    data: Document,
    options: Option<UpdateOptions>,
  ) -> Result<UpdateResult>
  where
    T: core::fmt::Debug + Schema,
  {
    self
      .builder
      .update_many_sync::<T>(conditions, data, options)
  }

  /// Updates _all_ documents in the database that match `conditions` without returning them.
  ///
  /// **Note** update_many will _not_ fire update middleware (`SchemaBefore::before_update()`).
  ///
  /// # Options
  /// ```rust,no_run,ignore
  /// UpdateOptions::builder()
  ///   // Optional (Vec<mongodb::bson::Document>)
  ///   // A set of filters specifying to which array elements an update should apply.
  ///   // See the documentation [here](https://docs.mongodb.com/manual/reference/command/update/) for more information on array filters.
  ///   .array_filters(...)
  ///   // Optional (bool)
  ///   // Opt out of document-level validation.
  ///   .bypass_document_validation(...)
  ///   // Optional (bool)
  ///   // If true, insert a document if no matching document is found.
  ///   .upsert(...)
  ///   // Optional (mongodb::options::Collation)
  ///   // The collation to use for the operation.
  ///   .collation(...)
  ///   // Optional (mongodb::options::Hint)
  ///   // A document or string that specifies the index to use to support the query predicate.
  ///   // Only available in MongoDB 4.2+. See the official MongoDB [documentation](https://docs.mongodb.com/manual/reference/command/update/#ex-update-command-hint) for examples.
  ///   .hint(...)
  ///   // Optional (mongodb::options::WriteConcern)
  ///   // The write concern for the operation.
  ///   .write_concern(...)
  ///   // Required to create the instance of `UpdateOptions`
  ///   .build()
  /// ```
  ///
  /// # Example
  /// ```rust,no_run,ignore
  /// // Update the age to 18 if it is under 18
  /// match nongoose.update_many::<User>(
  ///   doc! { "age": { "$lt": 18 } },
  ///   doc! { "$set": { "age": 18 } },
  ///   None
  /// ).await {
  ///   Ok(result) => println!("Modified {} documents", result.modified_count),
  ///   Err(error) => eprintln!("Error updating users: {}", error),
  /// }
  /// ```
  #[cfg(feature = "async")]
  pub async fn update_many<T>(
    &self,
    conditions: Document,
    data: Document,
    options: Option<UpdateOptions>,
  ) -> Result<UpdateResult>
  where
    T: core::fmt::Debug + Schema + 'static,
  {
    let builder = self.builder.clone();
    spawn_blocking(move || builder.update_many_sync::<T>(conditions, data, options)).await?
  }
}
