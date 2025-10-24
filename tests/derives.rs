mod common;

use common::*;
use derive_fields::{FieldKeys, Fields};

#[derive(Fields, FieldKeys)]
#[fields_derives(Debug)]
#[field_keys_derives(Debug)]
pub struct ExampleStruct {
    _name: String,
    _category: Category,
    _really_really_long_key: bool,
}

fn main() {
    // Confirm that key and field no longer implement clone
    let key = ExampleStructFieldKey::Name;
    let field = ExampleStructField::Name("Albert".to_string());
    let _ = key.clone();
    let _ = field.clone();
}
