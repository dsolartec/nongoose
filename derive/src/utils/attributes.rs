use syn::{Attribute, Meta, MetaList};

pub(crate) fn is_schema(attr: &Attribute) -> bool {
  attr.path.is_ident("schema")
}

pub(crate) fn parse(attr: &Attribute) -> MetaList {
  match attr.parse_meta() {
    Ok(Meta::List(list)) => list,
    _ => panic!("Invalid attribute syntax"),
  }
}
