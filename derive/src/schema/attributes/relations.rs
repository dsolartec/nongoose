use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parser, Data, DeriveInput, Field, Fields, Lit, Meta, NestedMeta};

pub(crate) fn parse(input: &mut DeriveInput) -> TokenStream {
  let nongoose = crate::utils::crates::get_nongoose_crate_name();

  match &mut input.data {
    Data::Struct(ref mut struct_data) => {
      match &mut struct_data.fields {
        Fields::Named(fields) => {
          for field in fields.named.clone().iter() {
            for attr in field.attrs.iter() {
              if !crate::utils::attributes::is_schema(attr) {
                continue;
              }

              let attr = crate::utils::attributes::parse(attr);
              for opt in attr.nested {
                match opt {
                  NestedMeta::Meta(Meta::NameValue(nv)) => {
                    if nv.path.is_ident("one_to_one") || nv.path.is_ident("many_to_one") {
                      if let Lit::Str(lit) = nv.lit {
                        let field_ident = field.ident.as_ref().unwrap();
                        let schema_ident = format_ident!("{}", lit.value());

                        if nv.path.is_ident("one_to_one") || nv.path.is_ident("many_to_one") {
                          let local_field_ident = format_ident!("{}_id", field_ident);

                          match Field::parse_named.parse2(quote! {
                            pub #local_field_ident: <#schema_ident as #nongoose::Schema>::__SchemaId
                          }) {
                            Ok(field) => fields.named.push(field),
                            Err(error) => panic!("{}", error),
                          }
                        }
                      }
                    }
                  }
                  _ => continue,
                }
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
