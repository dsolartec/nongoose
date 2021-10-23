use mongodb::bson::Bson;

use crate::errors::{Error, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchemaRelationType {
  OneToOne,
  OneToMany,
  ManyToOne,
}

impl SchemaRelationType {
  pub fn parse_str(text: &str) -> Result<SchemaRelationType> {
    match text {
      "one_to_one" => Ok(SchemaRelationType::OneToOne),
      "one_to_many" => Ok(SchemaRelationType::OneToMany),
      "many_to_one" => Ok(SchemaRelationType::ManyToOne),
      _ => Err(Error::NoImplemented),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SchemaRelation {
  pub field_ident: String,
  pub field_value: Bson,

  pub relation_type: SchemaRelationType,

  pub schema_ident: String,
  pub schema_name: String,
}

impl SchemaRelation {
  pub fn field_id(&self) -> String {
    format!("{}_id", self.field_ident)
  }
}
