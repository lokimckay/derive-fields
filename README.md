# Derive Fields

Macros to derive enums describing the fields of a struct

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
