use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::data::SchemaData;

pub(crate) fn getter<'a>(schema_data: &'a SchemaData) -> TokenStream {
  let mongodb = crate::utils::crates::get_mongodb_crate_name();
  let nongoose = crate::utils::crates::get_nongoose_crate_name();

  if schema_data.relations.len() > 0 {
    let mut relations = quote!();
    for (field_ident, relation_type, _schema_ident) in schema_data.relations.iter() {
      if relation_type == "one_to_many" {
        continue;
      }

      let local_field_name = format!("{}_id", quote!(#field_ident));
      relations.extend(quote! {
       (#local_field_name, self.#field_ident.clone().unwrap().__get_id()),
      });
    }

    if !relations.is_empty() {
      return quote! {
        fn __to_document(&self) -> #nongoose::errors::Result<#mongodb::bson::Document> {
          use #mongodb::bson::{to_bson, Bson};

          let mut document = match to_bson(self)? {
            Bson::Document(d) => d,
            _ => unreachable!(),
          };

          let relations_data = vec![#relations];
          for (field, data) in relations_data {
            document.insert(field, data);
          }

          Ok(document)
        }
      };
    }
  }

  quote! {
    fn __to_document(&self) -> #nongoose::errors::Result<#mongodb::bson::Document> {
      use #mongodb::bson::{to_bson, Bson};

      match to_bson(self)? {
        Bson::Document(d) => Ok(d),
        _ => unreachable!(),
      }
    }
  }
}
