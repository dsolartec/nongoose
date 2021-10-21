use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::data::SchemaData;

pub(crate) fn getter<'a>(schema_data: &'a SchemaData) -> TokenStream {
  let nongoose = crate::utils::crates::get_nongoose_crate_name();

  if schema_data.unique.len() > 0 {
    let mut idents = quote!();
    for field in schema_data.unique.iter() {
      let ident = field.ident.as_ref().unwrap();
      let ident_str = format!("{}", ident);

      let value = quote!(self.#ident);

      if let Some(lit) = schema_data.convert.get(ident) {
        let convert_ident = format_ident!("{}", lit.value());
        idents.extend(quote! {
          (
            #nongoose::re_exportsmongodb::bson::doc! { #ident_str: #convert_ident(#value.clone()) },
            #ident_str,
            #value.clone().to_string(),
          ),
        });
      } else {
        idents.extend(quote! {
          (
            #nongoose::re_exports::mongodb::bson::doc! { #ident_str: #value.clone() },
            #ident_str,
            #value.clone().to_string(),
          ),
        });
      }
    }

    quote! {
      fn __check_unique_fields(&self, database: &#nongoose::re_exports::mongodb::sync::Database) -> #nongoose::errors::Result<()> {
        let data = vec![#idents];
        for (document, field, value) in data {
          if database
            .collection::<#nongoose::re_exports::mongodb::bson::Document>(Self::__get_collection_name().as_str())
            .find_one(document, None)?
            .is_some()
          {
            return Err(#nongoose::errors::Error::DuplicatedSchemaField(field.to_string(), value.to_string()));
          }
        }

        Ok(())
      }
    }
  } else {
    quote! {
      fn __check_unique_fields(&self, _database: &#nongoose::re_exports::mongodb::sync::Database) -> #nongoose::errors::Result<()> {
        Ok(())
      }
    }
  }
}
