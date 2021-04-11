#[allow(unused_imports)]
use crate::Builder;

#[test]
fn emit_no_error() {
    #[derive(Builder)]
    #[allow(dead_code)]
    pub struct Person {
        name: String,
        age: u32,
        address: Option<String>,
    }
}

#[test]
fn call_builder() {
    #[allow(dead_code)]
    #[derive(Builder)]
    pub struct Person {
        name: String,
        age: u32,
        address: Option<String>,
    }
    let _ = Person::builder();
}
