use mongodb::bson::Bson;

use crate::error::{Error, Result};

/// Type of the relation with other Schema.
///
/// - [One-to-one](https://en.wikipedia.org/wiki/One-to-one_(data_model))
/// - [One-to-many](https://en.wikipedia.org/wiki/One-to-many_(data_model))
/// - Many-to-one
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchemaRelationType {
  /// One to one relationship.
  ///
  /// Read [One-to-one (Data model)](https://en.wikipedia.org/wiki/One-to-one_(data_model)) for more information.
  OneToOne,

  /// One to many relationship.
  ///
  /// Read [One-to-many (Data model)](https://en.wikipedia.org/wiki/One-to-many_(data_model)) for more information.
  OneToMany,

  /// Many to one relationship.
  ManyToOne,
}

impl SchemaRelationType {
  /// Parse a str to `SchemaRelationType` enum.
  ///
  /// # Example
  /// ```rust
  /// use nongoose::types::SchemaRelationType;
  ///
  /// let one_to_one = SchemaRelationType::parse_str("one_to_one");
  /// assert!(one_to_one.is_ok());
  /// assert_eq!(one_to_one.unwrap(), SchemaRelationType::OneToOne);
  ///
  /// let no_implemented = SchemaRelationType::parse_str("other_relation");
  /// assert!(no_implemented.is_err());
  /// ```
  pub fn parse_str(text: &str) -> Result<SchemaRelationType> {
    match text {
      "one_to_one" => Ok(SchemaRelationType::OneToOne),
      "one_to_many" => Ok(SchemaRelationType::OneToMany),
      "many_to_one" => Ok(SchemaRelationType::ManyToOne),
      _ => Err(Error::NoImplemented),
    }
  }
}

/// Schema relation data.
#[derive(Clone, Debug, PartialEq)]
pub struct SchemaRelation {
  /// Name of the field inside the Schema struct.
  pub field_ident: String,

  /// Value of the field.
  pub field_value: Bson,

  /// Type of the relation with the Schema.
  pub relation_type: SchemaRelationType,

  /// Name of the Schema to relate.
  pub schema_ident: String,

  /// Schema name in the database.
  pub schema_name: String,
}

impl SchemaRelation {
  /// Name of the field in the database (`self.field_ident`_id).
  pub fn field_id(&self) -> String {
    format!("{}_id", self.field_ident)
  }
}
