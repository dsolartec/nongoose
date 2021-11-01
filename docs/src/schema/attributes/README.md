# Schema Attributes

[Attributes](https://doc.rust-lang.org/book/attributes.html) are used to customize the `Schema` implementation produced by Nongoose's derive. They require a Rust compiler version 1.15 or newer.

There are three categories of attributes:

- [Macro attributes](./macro.md) - apply to a struct declaration before `Schema` derive.
- [Container attributes]() - apply to a struct declaration.
- [Field attributes]() - apply to one field in a struct variant.

```rust,no_run
#[schema_relations] // <-- this is a macro attribute
#[derive(Clone, Debug, Deserialize, Schema, Serialize)]
#[schema(name = "users")]   // <-- this is a container attribute
struct User {
  #[schema(id)] // <-- this is a field attribute
  #[serde(rename = "_id")]
  id: ObjectId,
}
```
