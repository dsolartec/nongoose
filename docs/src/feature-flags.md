# Feature flags

The `nongoose` crate defines some [Cargo features](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section) to enable using Nongoose in a variety of freestanding environments.

## --feature derive

Provide derive and attributes macros for the `Schema` trait.

This is behind a feature because the derive macro implementation takes some extra time to compile.

## --feature async

Provide asynchronous functions in the `Schema` trait using [Tokio](https://tokio.rs) runtime.
