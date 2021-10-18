use syn::{Data, DeriveInput, Fields, FieldsNamed};

pub(crate) fn get(input: &DeriveInput) -> &FieldsNamed {
  match &input.data {
    Data::Struct(s) => match &s.fields {
      Fields::Named(fields) => fields,
      _ => panic!("Schema only supports named fields"),
    },
    _ => panic!("Schema only supports named fields"),
  }
}
