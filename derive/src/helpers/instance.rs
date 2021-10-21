use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn getter<'a>() -> TokenStream {
  let nongoose = crate::utils::crates::get_nongoose_crate_name();

  quote! {
    fn __get_instance(instance: Option<#nongoose::NongooseBuilder>) -> &'static ::std::sync::Mutex<#nongoose::NongooseBuilder> {
      use ::std::sync::Mutex;
      use #nongoose::{re_exports::OnceCell, NongooseBuilder};

      let collection_name = Self::__get_collection_name();

      static INSTANCE: OnceCell<Mutex<NongooseBuilder>> = OnceCell::new();

      if let Some(instance_mut) = INSTANCE.get() {
        let mut instance_ctx = instance_mut.lock().unwrap();
        if instance_ctx.schemas.contains(&collection_name) {
          return instance_mut;
        } else if let Some(instance) = instance {
          instance_ctx.replace_with(instance);
          return instance_mut;
        }
      } else if let Some(instance) = instance {
        INSTANCE.set(Mutex::new(instance)).unwrap();
        return INSTANCE.get().unwrap();
      }

      panic!(
        "Schema is not associated to a Nongoose instance ({})",
        collection_name
      );
    }
  }
}
