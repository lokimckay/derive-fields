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

## Testing

`cargo test --test tests --all-features`
