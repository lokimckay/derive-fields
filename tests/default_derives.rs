mod common;

use common::*;
use derive_fields::{FieldKeys, Fields};
use std::collections::HashSet;

#[derive(FieldKeys, Fields)]
pub struct ExampleStruct {
    _name: String,
    _category: Category,
    _really_really_long_key: bool,
}

fn main() {
    // Confirm keys implement Debug + Clone + Copy + PartialEq + Eq + Hash
    let key1 = ExampleStructFieldKey::Name;
    let key2 = ExampleStructFieldKey::Name;
    let key3 = key1.clone();
    let key4 = key2;
    let _ = format!("{:?}", key1);
    assert_eq!(key3, key4);
    let mut set = HashSet::new();
    set.insert(key4);

    // Confirm fields implement Debug + Clone
    let field = ExampleStructField::Name("Albert".to_string());
    let _ = format!("{:?}", field.clone());
}
