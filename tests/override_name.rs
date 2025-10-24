mod common;

use common::*;
use derive_fields::{FieldKeys, Fields};

#[derive(Fields, FieldKeys)]
#[fields_name(FooField)]
#[field_keys_name(BarKey)]
pub struct ExampleStruct {
    _name: String,
    _category: Category,
    _really_really_long_key: bool,
}

fn main() {
    let _ = BarKey::Name;
    let _ = FooField::Name("Albert".to_string());
}
