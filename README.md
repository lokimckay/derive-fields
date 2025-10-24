## Usage

The available usages and their expected outputs are listed below.

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
