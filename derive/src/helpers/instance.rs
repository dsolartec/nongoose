use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn getter() -> TokenStream {
  let nongoose = crate::utils::crates::get_nongoose_crate_name();

  quote! {
    fn __get_database(database: Option<#nongoose::Database>) -> &'static #nongoose::Database {
      use #nongoose::re_exports::OnceCell;

      let collection_name = Self::collection_name();

      static DATABASE: OnceCell<#nongoose::Database> = OnceCell::new();

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
