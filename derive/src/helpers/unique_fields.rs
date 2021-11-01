use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::data::SchemaData;

pub(crate) fn getter(schema_data: &SchemaData) -> TokenStream {
  let nongoose = crate::utils::crates::get_nongoose_crate_name();

  if !schema_data.unique.is_empty() {
    let mut idents = quote!();
    for field in schema_data.unique.iter() {
      let ident = field.ident.as_ref().unwrap();
      let ident_str = format!("{}", ident);

      let value = quote!(self.#ident);

      if let Some(lit) = schema_data.convert.get(ident) {
        let convert_ident = format_ident!("{}", lit.value());
        idents.extend(quote! {
          (
            #nongoose::mongodb::bson::doc! { #ident_str: #convert_ident(#value.clone()) },
            #ident_str,
            #value.clone().to_string(),
          ),
        });
      } else {
        idents.extend(quote! {
          (
            #nongoose::mongodb::bson::doc! { #ident_str: #value.clone() },
            #ident_str,
            #value.clone().to_string(),
          ),
        });
      }
    }

    quote! {
      fn __check_unique_fields(&self) -> #nongoose::errors::Result<()> {
        let data = vec![#idents];
        for (document, field, value) in data {
          if let Some(doc) = Self::__get_database(None)
            .collection::<#nongoose::mongodb::bson::Document>(Self::__get_collection_name().as_str())
            .find_one(document, None)?
          {
            let data: Self = #nongoose::mongodb::bson::from_bson(#nongoose::mongodb::bson::Bson::Document(doc))?;
            if self.__get_id() != data.__get_id() {
              return Err(#nongoose::errors::Error::DuplicatedSchemaField(field.to_string(), value.to_string()));
            }
          }
        }

        Ok(())
      }
    }
  } else {
    quote! {
      fn __check_unique_fields(&self) -> #nongoose::errors::Result<()> {
        Ok(())
      }
    }
  }
}
