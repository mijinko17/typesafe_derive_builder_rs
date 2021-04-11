use syn::{parse_macro_input, DeriveInput};
use typesafe_derive_builder_core::builder_token_stream;

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    builder_token_stream(ast).into()
}
