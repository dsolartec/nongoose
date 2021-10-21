use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use proc_macro_crate::{crate_name, FoundCrate};

pub(crate) fn get_nongoose_crate_name() -> TokenStream {
  let name = match crate_name("nongoose") {
    Ok(FoundCrate::Name(name)) => name,
    Ok(FoundCrate::Itself) | Err(_) => "nongoose".to_string(),
  };

  TokenTree::from(Ident::new(&name, Span::call_site())).into()
}
