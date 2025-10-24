# Derive Fields

Macros to derive enums describing the fields of a struct.  
Useful if you want to partially represent a struct in something like a hashmap.

## Usage

Add as a dependency to your `Cargo.toml`:

```toml
derive-fields = { git = "https://github.com/lokimckay/derive-fields.git", branch = "main" }
```

### `#[derive(Fields)]`

```rs
#[derive(Fields)]
struct MyStruct {
    name: String,
    category: Category,
    really_really_long_key: bool,
}
```

```rs
pub enum MyStructField {
    Name(String),
    Category(Category),
    ReallyReallyLongKey(bool),
}
```

### `#[derive(FieldKeys)]`

```rs
#[derive(FieldKeys)]
struct MyStruct {
    name: String,
    category: Category,
    really_really_long_key: bool,
}
```

```rs
pub enum MyStructFieldKey {
    Name,
    Category,
    ReallyReallyLongKey,
}
```

### Adding derives to generated enums

By default,

- `*FieldKeys` implements `Debug + Clone + Copy + PartialEq + Eq + Hash`
- `*Field` implements `Debug + Clone`

You can override these defaults by adding `#[field_keys_derives(...)]` or `#[fields_derives(...)]` to the struct.

#### `#[field_keys_derives(...)]`

Adds derives to the generated `*FieldKeys` enum.

```rs
#[derive(FieldKeys)]
#[field_keys_derives(Debug)]
pub struct ExampleStruct;
```

Generates:

```rs
#[derive(Debug)]
pub enum ExampleStructFieldKey;
```

#### `#[fields_derives(...)]`

Adds derives to the generated `*Field` enum.

```rs
#[derive(Fields)]
#[fields_derives(Debug)]
pub struct ExampleStruct;
```

Generates:

```rs
#[derive(Debug)]
pub enum ExampleStructField;
```

### Aliasing generated enums

By default, the generated enums are named `*Field` and `*FieldKey`.  
You add an additional type alias by adding `#[fields_alias(...)]` or `#[field_keys_alias(...)]` to the struct.

```rs
#[derive(Fields, FieldKeys)]
#[fields_alias(FooField)]
#[field_keys_alias(BarKey)]
pub struct ExampleStruct;
```

Generates:

```rs
pub type FooField = ExampleStructField;
pub type BarKey = ExampleStructFieldKey;
```

### Serde support

If you enable the `keys-serde` or `fields-serde` features, the corresponding enums will automatically implement `serde::Serialize` and `serde::Deserialize`.

## Testing

`cargo test --test tests --all-features`
