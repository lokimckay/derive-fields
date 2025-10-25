mod common;

use common::*;
use derive_fields::FieldKeys;

#[derive(FieldKeys)]
pub struct ExampleStruct {
    _name: String,
    _category: Category,
    _really_really_long_key: bool,
}

fn main() {
    for field in ExampleStructFieldKey::iter() {
        let _ = format!("{field:?}");
    }
}
