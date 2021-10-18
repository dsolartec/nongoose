use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::data::SchemaData;

pub(crate) fn getter<'a>(schema_data: &'a SchemaData) -> TokenStream {
  let mongodb = crate::utils::crates::get_mongodb_crate_name();

  if schema_data.relations.len() > 0 {
    let mut relations = quote!();
    for (field_ident, relation_type, schema_ident) in schema_data.relations.iter() {
      let schema_ident = format_ident!("{}", schema_ident.value());

      if relation_type == "one_to_one" || relation_type == "many_to_one" {
        let field_ident_name = format!("{}", quote!(#field_ident));
        let local_field_name = format!("{}_id", field_ident_name);

        relations.extend(quote! {
          #mongodb::bson::doc! {
            "$lookup": {
              "from": <#schema_ident>::__get_collection_name(),
              "localField": #local_field_name,
              "foreignField": "_id",
              "as": #field_ident_name,
            },
          },
          #mongodb::bson::doc! {
            "$set": {
              #field_ident_name: {
                "$first": format!("${}", #field_ident_name)
              }
            }
          },
        });
      }
    }

    if !relations.is_empty() {
      return quote! {
        fn __get_relations() -> Option<Vec<#mongodb::bson::Document>> {
          let relations = vec![#relations];
          if !relations.is_empty() {
            Some(relations)
          } else {
            None
          }
        }
      };
    }
  }

  quote! {
    fn __get_relations() -> Option<Vec<#mongodb::bson::Document>> {
      None
    }
  }
}
