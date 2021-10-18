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
  let schema_id_getter = crate::helpers::schema_id::getter(&schema_data);
  let unique_fields_getter = crate::helpers::unique_fields::getter(&schema_data);
  let document_getter = crate::helpers::document::getter(&schema_data);
  let relations_getter = crate::helpers::relations::getter(&schema_data);

  let traits = quote! {
    #[#nongoose::re_exports::async_trait]
    impl #nongoose::Schema for #ident {
      fn __get_collection_name() -> String {
        #collection_name.to_string()
      }

      #schema_id_getter
      #unique_fields_getter
      #document_getter
      #relations_getter
    }
  };

  traits.into()
}
