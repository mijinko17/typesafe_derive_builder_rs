use quote::quote;
use syn::DeriveInput;

pub fn builder_token_stream(_: DeriveInput) -> proc_macro2::TokenStream {
    quote! {}
}

#[cfg(test)]
mod tests {}
