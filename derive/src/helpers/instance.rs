use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn getter<'a>() -> TokenStream {
  let nongoose = crate::utils::crates::get_nongoose_crate_name();

  quote! {
    fn __get_database(database: Option<#nongoose::mongodb::sync::Database>) -> &'static #nongoose::mongodb::sync::Database {
      use #nongoose::re_exports::OnceCell;

      let collection_name = Self::__get_collection_name();

      static DATABASE: OnceCell<#nongoose::mongodb::sync::Database> = OnceCell::new();

      if let Some(database) = DATABASE.get() {
        return database;
      } else if let Some(database) = database {
        DATABASE.set(database).unwrap();
        return DATABASE.get().unwrap();
      }

      panic!(
        "Schema is not associated to a Nongoose instance ({})",
        collection_name
      );
    }
  }
}
