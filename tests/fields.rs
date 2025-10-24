mod common;

use common::*;
use derive_fields::Fields;

#[derive(Fields)]
#[fields_derives(Debug, PartialEq, Eq)]
pub struct ExampleStruct {
    _name: String,
    _category: Category,
    _really_really_long_key: bool,
}

fn main() {
    let _ = ExampleStructField::Name("Albert".to_string());
    let _ = ExampleStructField::Category(Category::CategoryA);
    let _ = ExampleStructField::ReallyReallyLongKey(true);
}
