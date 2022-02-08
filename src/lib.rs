//! # Nongoose
//!
//! Nongoose is an Object Data Mapping (ODM) for MongoDB based on Mongoose.
//!
//! # Installation
//!
//! ## Requeriments
//! - Rust 1.48+
//! - MongoDB 3.6+
//!
//! ## Importing
//! The library is available on [crates.io](https://crates.io/crates/nongoose). To use it in
//! your application, simply add it to your project's `Cargo.toml`.
//! ```toml
//! [dependencies]
//! nongoose = "0.1.0-beta.1"
//! ```
//!
//! ### All feature flags
//!
//! | Feature         | Description                                                                                                       | Extra dependencies                                      | Default |
//! |:----------------|:------------------------------------------------------------------------------------------------------------------|:--------------------------------------------------------|:--------|
//! | `derive`        | Enable support for the macro derives                                                                              | `nongoose-derive`                                       | yes     |
//! | `sync`          | Expose the synchronous API. This flag cannot be used in conjuntion with either of the async runtime feature flags | n/a                                                     | no      |
//! | `tokio-runtime` | Enable support for the `tokio` async runtime                                                                      | `tokio` 1.0 with the `macros` feature and `async-trait` | yes     |

#![warn(missing_docs)]
#![deny(unused_imports)]
#![doc(html_root_url = "https://docs.rs/nongoose/0.1.0-beta.1")]

mod error;
mod nongoose;
#[doc(hidden)]
pub mod re_exports;
mod schema;

pub use crate::nongoose::{Nongoose, NongooseBuilder};
pub use error::{Error, Result};
pub use mongodb::bson;
pub use mongodb::options;
pub use mongodb::sync::{Client, Database};
#[cfg(feature = "derive")]
pub use nongoose_derive::{schema_relations, Schema};
pub use schema::{types, Schema, SchemaBefore};
