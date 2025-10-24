mod common;

use common::*;
use derive_fields::{FieldKeys, Fields};
use std::collections::HashMap;

use ExampleStructField as Value;
use ExampleStructFieldKey as Key;

#[derive(FieldKeys, Fields)]
#[fields_derives(Debug, Clone)]
#[field_keys_derives(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExampleStruct {
    _name: String,
    _category: Category,
    _really_really_long_key: bool,
}

impl From<Value> for Key {
    fn from(field: Value) -> Self {
        match field {
            Value::Name(_) => Key::Name,
            Value::Category(_) => Key::Category,
            Value::ReallyReallyLongKey(_) => Key::ReallyReallyLongKey,
        }
    }
}

fn main() {
    let value = Value::Name("Albert".to_string());

    let mut map: HashMap<Key, Value> = HashMap::new();
    map.insert(value.clone().into(), value);
}
