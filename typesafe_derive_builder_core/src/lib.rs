use quote::quote;
use syn::DeriveInput;

pub fn builder_token_stream(_: DeriveInput) -> proc_macro2::TokenStream {
    quote! {
        impl Person {
            pub fn builder() -> PersonBuilder<(), ()> {
                PersonBuilder {
                    name: (),
                    age: (),
                    address: None,
                }
            }
        }

        pub struct PersonBuilder<Name, Age> {
            name: Name,
            age: Age,
            address: Option<String>,
        }
        impl<Name, Age> PersonBuilder<Name, Age> {
            pub fn name(self, name: &str) -> PersonBuilder<String, Age> {
                PersonBuilder {
                    name: name.to_string(),
                    age: self.age,
                    address: self.address,
                }
            }
            pub fn age(self, age: u32) -> PersonBuilder<Name, u32> {
                PersonBuilder {
                    name: self.name,
                    age: age,
                    address: self.address,
                }
            }
            pub fn address(mut self, address: &str) -> PersonBuilder<Name, Age> {
                self.address = Some(address.to_string());
                self
            }
        }
        impl PersonBuilder<String, u32> {
            pub fn build(self) -> Person {
                Person {
                    name: self.name,
                    age: self.age,
                    address: self.address,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {}
