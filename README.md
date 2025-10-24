# Derive Fields

Macros to derive enums describing the fields of a struct

## Usage

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
