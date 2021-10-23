use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parser, Data, DeriveInput, Field, Fields, Lit, Meta, NestedMeta};

pub(crate) fn parse(input: &mut DeriveInput) -> TokenStream {
  let nongoose = crate::utils::crates::get_nongoose_crate_name();

  match &mut input.data {
    Data::Struct(ref mut struct_data) => {
      match &mut struct_data.fields {
        Fields::Named(fields) => {
          let mut optional_relation_fields = Vec::new();
          let mut relation_fields = Vec::new();

          let fields_named = fields.named.clone();
          for field in fields_named.iter() {
            for attr in field.attrs.iter() {
              if !crate::utils::attributes::is_schema(attr) {
                continue;
              }

              let attr = crate::utils::attributes::parse(attr);
              for opt in attr.nested {
                match opt {
                  NestedMeta::Meta(Meta::Path(path)) => {
                    if path.is_ident("optional") {
                      optional_relation_fields.push(field);
                    }
                  }
                  NestedMeta::Meta(Meta::NameValue(nv)) => {
                    if nv.path.is_ident("one_to_one") || nv.path.is_ident("many_to_one") {
                      if let Lit::Str(lit) = nv.lit {
                        let field_ident = field.ident.as_ref().unwrap();
                        let schema_ident = format_ident!("{}", lit.value());

                        if nv.path.is_ident("one_to_one") || nv.path.is_ident("many_to_one") {
                          let local_field_ident = format_ident!("{}_id", field_ident);
                          relation_fields.push((field, local_field_ident, schema_ident));
                        }
                      }
                    }
                  }
                  _ => continue,
                }
              }
            }
          }

          if !relation_fields.is_empty() {
            for (relation_field, local_field_ident, schema_ident) in relation_fields.iter() {
              let mut quote_data = quote!(pub #local_field_ident: );

              if optional_relation_fields.contains(relation_field) {
                quote_data.extend(quote!(Option<<#schema_ident as #nongoose::Schema>::__SchemaId>));
              } else {
                quote_data.extend(quote!(<#schema_ident as #nongoose::Schema>::__SchemaId));
              }

              match Field::parse_named.parse2(quote_data) {
                Ok(field) => fields.named.push(field),
                Err(error) => panic!("{}", error),
              }
            }
          }
        }
        _ => (),
      }

      return quote! { #input };
    }
    _ => panic!("Schema only supports named fields"),
  }
}
