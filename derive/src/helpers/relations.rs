use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::data::SchemaData;

pub(crate) fn getter(schema_data: &SchemaData) -> TokenStream {
  let nongoose = crate::utils::crates::get_nongoose_crate_name();

  if !schema_data.relations.is_empty() {
    let mut static_relations = quote!();
    let mut get_relations = quote!();
    let mut set_relations = quote!();

    for (field_ident, relation_type, schema_ident) in schema_data.relations.iter() {
      let field_ident_name = format!("{}", quote!(#field_ident));
      let field_id_ident = format_ident!("{}_id", field_ident_name);

      let schema_ident_name = schema_ident.value().to_string();
      let schema_ident = format_ident!("{}", schema_ident_name);

      static_relations.extend(quote! {
        #nongoose::types::SchemaRelation {
          field_ident: #field_ident_name.to_string(),
          field_value: #nongoose::bson::Bson::Null,

          relation_type: #nongoose::types::SchemaRelationType::parse_str(#relation_type).unwrap(),

          schema_ident: #schema_ident_name.to_string(),
          schema_name: <#schema_ident>::collection_name(),
        },
      });

      if relation_type == "one_to_one" || relation_type == "many_to_one" {
        get_relations.extend(quote! {
          #nongoose::types::SchemaRelation {
            field_ident: #field_ident_name.to_string(),
            field_value: if let Some(field_data) = self.#field_ident.clone() {
              field_data.__get_id().into()
            } else {
              self.#field_id_ident.clone().into()
            },

            relation_type: #nongoose::types::SchemaRelationType::parse_str(#relation_type).unwrap(),

            schema_ident: #schema_ident_name.to_string(),
            schema_name: <#schema_ident>::collection_name(),
          },
        });
      } else if relation_type == "one_to_many" {
        get_relations.extend(quote! {
          #nongoose::types::SchemaRelation {
            field_ident: #field_ident_name.to_string(),
            field_value: self.#field_ident.clone().into(),

            relation_type: #nongoose::types::SchemaRelationType::parse_str(#relation_type).unwrap(),

            schema_ident: #schema_ident_name.to_string(),
            schema_name: <#schema_ident>::collection_name(),
          },
        });
      }

      if !set_relations.is_empty() {
        set_relations.extend(quote!(else));
      }

      set_relations.extend(quote! {
        if field == #field_ident_name {
          self.#field_ident = #nongoose::bson::from_bson(new_value)?;
          return Ok(());
        }
      });
    }

    if !static_relations.is_empty() && !get_relations.is_empty() && !set_relations.is_empty() {
      return quote! {
        fn __relations() -> Vec<#nongoose::types::SchemaRelation> {
          vec![#static_relations]
        }

        fn __get_relations(&self) -> Option<Vec<#nongoose::types::SchemaRelation>> {
          let relations = vec![#get_relations];
          if !relations.is_empty() {
            Some(relations)
          } else {
            None
          }
        }

        fn __set_relations(&mut self, field: &str, new_value: #nongoose::bson::Bson) -> #nongoose::Result<()> {
          #set_relations
          Err(#nongoose::Error::NoImplemented)
        }
      };
    }
  }

  quote! {
    fn __relations() -> Vec<#nongoose::types::SchemaRelation> {
      Vec::new()
    }

    fn __get_relations(&self) -> Option<Vec<#nongoose::types::SchemaRelation>> {
      None
    }

    fn __set_relations(&mut self, _field: &str, _new_value: #nongoose::bson::Bson) -> #nongoose::Result<()> {
      Ok(())
    }
  }
}
