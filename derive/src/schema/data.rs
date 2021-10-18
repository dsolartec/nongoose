use std::collections::HashMap;

use proc_macro2::Ident;
use syn::{Field, FieldsNamed, Lit, LitStr, Meta, NestedMeta};

pub(crate) struct SchemaData<'a> {
  pub convert: HashMap<&'a Ident, LitStr>,
  pub id: &'a Field,
  pub unique: Vec<&'a Field>,
  pub relations: Vec<(&'a Ident, String, LitStr)>,
}

pub(crate) fn parse_fields<'a>(fields: &'a FieldsNamed) -> SchemaData<'a> {
  let mut convert = HashMap::new();
  let mut id = None;
  let mut unique = Vec::new();
  let mut relations = Vec::new();

  for field in &fields.named {
    for attr in &field.attrs {
      if !crate::utils::attributes::is_schema(attr) {
        continue;
      }

      let attr = crate::utils::attributes::parse(attr);
      for opt in attr.nested {
        match opt {
          NestedMeta::Meta(Meta::Path(path)) => {
            if path.is_ident("id") {
              if id.is_some() {
                panic!("Schema only supports one id field");
              } else {
                id = Some(field);
                unique.push(field);
              }
            } else if path.is_ident("unique") {
              if !unique.contains(&field) {
                unique.push(field);
              }
            }
          }
          NestedMeta::Meta(Meta::NameValue(nv)) => {
            let field_ident = field.ident.as_ref().unwrap();

            if nv.path.is_ident("convert") {
              if let Lit::Str(lit) = nv.lit {
                convert.insert(field_ident, lit);
              }
            } else if nv.path.is_ident("one_to_one") || nv.path.is_ident("many_to_one") {
              if let Lit::Str(lit) = nv.lit {
                relations.push((
                  field_ident,
                  nv.path.get_ident().as_ref().unwrap().to_string(),
                  lit,
                ));
              }
            }
          }
          _ => continue,
        }
      }
    }
  }

  if id.is_none() {
    panic!("Schema needs an id field");
  }

  SchemaData {
    convert,
    id: id.unwrap(),
    unique,
    relations,
  }
}
