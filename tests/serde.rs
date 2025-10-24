mod common;

use common::*;
use derive_fields::{FieldKeys, Fields};

#[derive(Fields, FieldKeys)]
pub struct ExampleStruct {
    _name: String,
    _category: Category,
    _really_really_long_key: bool,
}

fn main() {
    let key = ExampleStructFieldKey::Name;
    let field = ExampleStructField::Name("Albert".to_string());

    let key_json = serde_json::to_string(&key).unwrap();
    let field_json = serde_json::to_string(&field).unwrap();

    assert_eq!(key_json, r#""Name""#);
    assert_eq!(field_json, r#"{"Name":"Albert"}"#);
}
