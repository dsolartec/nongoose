use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::data::SchemaData;

pub(crate) fn getter(schema_data: &SchemaData<'_>) -> TokenStream {
  let id_field_ident = schema_data.id.ident.as_ref().unwrap();

  if let Some(lit) = schema_data.convert.get(id_field_ident) {
    let nongoose = crate::utils::crates::get_nongoose_crate_name();
    let convert_ident = format_ident!("{}", lit.value());

    quote! {
      type Id = #nongoose::bson::Bson;

      fn __get_id(&self) -> Self::Id {
        #convert_ident(self.#id_field_ident.clone())
      }
    }
  } else {
    let id_field_type = &schema_data.id.ty;

    quote! {
      type Id = #id_field_type;

      fn __get_id(&self) -> Self::Id {
        self.#id_field_ident.clone()
      }
    }
  }
}
