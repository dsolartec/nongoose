pub(crate) mod attributes;
pub(crate) mod data;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn parse(input: &DeriveInput) -> TokenStream {
  let collection_name = crate::utils::collection::get_name(input);
  let ident = &input.ident;

  let fields = crate::utils::fields::get(input);
  let schema_data = data::parse_fields(fields);

  let nongoose = crate::utils::crates::get_nongoose_crate_name();

  // Helpers
  let instance_getter = crate::helpers::instance::getter();
  let schema_id_getter = crate::helpers::schema_id::getter(&schema_data);
  let unique_fields_getter = crate::helpers::unique_fields::getter(&schema_data);
  let relations_getter = crate::helpers::relations::getter(&schema_data);

  let traits = quote! {
    #[cfg_attr(feature = "async", #nongoose::re_exports::async_trait)]
    impl #nongoose::Schema for #ident {
      fn __get_collection_name() -> String {
        #collection_name.to_string()
      }

      #instance_getter
      #schema_id_getter
      #unique_fields_getter
      #relations_getter
    }

    impl From<#ident> for #nongoose::mongodb::bson::Bson {
      fn from(key: #ident) -> Self {
        match #nongoose::mongodb::bson::to_bson(&key) {
          Ok(bson) => bson,
          Err(e) => panic!("Cannot parse the Schema as BSON (name: {})", <#ident as #nongoose::Schema>::__get_collection_name()),
        }
      }
    }
  };

  traits.into()
}
