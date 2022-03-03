use crate::types::SchemaRelation;

#[derive(Clone, Debug, PartialEq)]
pub struct SchemaData {
  name: String,
  relations: Vec<SchemaRelation>,
}

impl SchemaData {
  pub(crate) fn new<T>() -> Self
  where
    T: super::Schema,
  {
    Self {
      name: T::collection_name(),
      relations: T::__relations(),
    }
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_relations(&self) -> Vec<SchemaRelation> {
    self.relations.clone()
  }
}
