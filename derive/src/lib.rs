use proc_macro::TokenStream;

pub(crate) mod helpers;
pub(crate) mod schema;
pub(crate) mod utils;

#[proc_macro_derive(Schema, attributes(schema))]
pub fn schema(tokens: TokenStream) -> TokenStream {
  let input = syn::parse(tokens).unwrap();
  schema::parse(&input)
}
