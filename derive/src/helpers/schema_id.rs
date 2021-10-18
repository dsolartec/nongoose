use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::data::SchemaData;

pub(crate) fn getter<'a>(schema_data: &'a SchemaData<'a>) -> TokenStream {
  let id_field_ident = schema_data.id.ident.as_ref().unwrap();

  if let Some(lit) = schema_data.convert.get(id_field_ident) {
    let convert_ident = format_ident!("{}", lit.value());
    let mongodb = crate::utils::crates::get_mongodb_crate_name();

    quote! {
      type __SchemaId = #mongodb::bson::Bson;

      fn __get_id(&self) -> Self::__SchemaId {
        #convert_ident(self.#id_field_ident.clone())
      }
    }
  } else {
    let id_field_type = &schema_data.id.ty;

    quote! {
      type __SchemaId = #id_field_type;

      fn __get_id(&self) -> Self::__SchemaId {
        self.#id_field_ident.clone()
      }
    }
  }
}
