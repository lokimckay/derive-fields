# Derive Fields

Macros to derive enums describing the fields of a struct.  
Useful if you want to partially represent a struct in something like a hashmap.

## Usage

1.  Add as a dependency to your `Cargo.toml`

    ```toml
    derive-fields = { git = "https://github.com/lokimckay/derive-fields.git", branch = "main" }
    ```

2.  Add one or both derive macros `Fields`, `FieldKeys` to your struct

    ```rs
    #[derive(Fields, FieldKeys)]
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

    pub enum MyStructFieldKey {
        Name,
        Category,
        ReallyReallyLongKey,
    }
    ```

## Configuration

### Adding derives to generated enums

By default,

- `*Field` implements `Debug + Clone`
- `*FieldKeys` implements `Debug + Clone + Copy + PartialEq + Eq + Hash`

You can override these defaults by adding `#[field_keys_derives(...)]` or `#[fields_derives(...)]` to the struct.

```rs
#[derive(Fields, FieldKeys)]
#[fields_derives(Debug)]
#[field_keys_derives(Debug)]
pub struct ExampleStruct;
```

```rs
#[derive(Debug)]
pub enum ExampleStructField;

#[derive(Debug)]
pub enum ExampleStructFieldKey;
```

### Renaming generated enums

By default, the generated enums are named `*Field` and `*FieldKey`.  
You can override these by adding `#[fields_name(...)]` or `#[field_keys_name(...)]` to the struct.

```rs
#[derive(Fields, FieldKeys)]
#[fields_name(FooField)]
#[field_keys_name(BarKey)]
pub struct ExampleStruct;
```

```rs
pub enum FooField { ... }
pub enum BarKey { ... }
```

### Serde support

If you enable the `keys-serde` or `fields-serde` features, the corresponding enums will automatically implement `serde::Serialize` and `serde::Deserialize`.

## Testing

`cargo test --test tests --all-features`
