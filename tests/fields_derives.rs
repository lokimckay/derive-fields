mod common;

use common::*;
use derive_fields::Fields;

#[derive(Fields)]
#[fields_derives(Debug)]
pub struct ExampleStruct {
    _name: String,
    _category: Category,
    _really_really_long_key: bool,
}

fn main() {
    // Confirm that field no longer implements clone
    let field = ExampleStructField::Name("Albert".to_string());
    let _ = field.clone();
}
