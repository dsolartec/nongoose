use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub(crate) mod helpers;
pub(crate) mod schema;
pub(crate) mod utils;

#[proc_macro_derive(Schema, attributes(schema))]
pub fn schema(tokens: TokenStream) -> TokenStream {
  let input = syn::parse(tokens).unwrap();
  schema::parse(&input)
}

#[proc_macro_attribute]
pub fn schema_relations(_args: TokenStream, input: TokenStream) -> TokenStream {
  let mut ast = parse_macro_input!(input as DeriveInput);
  schema::attributes::relations::parse(&mut ast).into()
}
