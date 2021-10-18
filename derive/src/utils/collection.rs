use syn::{DeriveInput, Lit, Meta, NestedMeta};

pub(crate) fn get_name(input: &DeriveInput) -> String {
  let attrs = &input.attrs;
  let mut result = None;

  for attr in attrs {
    if !super::attributes::is_schema(attr) {
      continue;
    }

    let attr = super::attributes::parse(attr);
    for opt in attr.nested {
      let nv = match opt {
        NestedMeta::Meta(Meta::NameValue(nv)) => nv,
        _ => continue,
      };

      if nv.path.is_ident("name") {
        match nv.lit {
          Lit::Str(lit) => result = Some(lit.value()),
          _ => continue,
        }
      }
    }
  }

  result.unwrap_or_else(|| {
    let ident = input.ident.to_string();
    let mut new_ident = String::new();

    for letter in ident.as_bytes().iter() {
      if letter.is_ascii_uppercase() {
        if new_ident.len() > 0 {
          new_ident.push('_');
        }

        new_ident.push(letter.to_ascii_lowercase() as char);
      } else {
        new_ident.push(letter.clone() as char);
      }
    }

    format!("{}s", new_ident)
  })
}
