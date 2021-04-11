#[allow(unused_imports)]
use crate::Builder;

#[test]
fn emit_no_error() {
    #[derive(Builder)]
    #[allow(dead_code)]
    struct Hoge {
        name: String,
        age: u32,
    }
}
