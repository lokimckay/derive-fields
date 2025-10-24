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
    let _ = ExampleStructFieldKey::Name;
    let _ = ExampleStructFieldKey::Category;
    let _ = ExampleStructFieldKey::ReallyReallyLongKey;
}
