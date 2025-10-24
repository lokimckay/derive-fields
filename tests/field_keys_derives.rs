mod common;

use common::*;
use derive_fields::FieldKeys;

#[derive(FieldKeys)]
#[field_keys_derives(Debug)]
pub struct ExampleStruct {
    _name: String,
    _category: Category,
    _really_really_long_key: bool,
}

fn main() {
    // Confirm that key no longer implements clone
    let key = ExampleStructFieldKey::Name;
    let _ = key.clone();
}
