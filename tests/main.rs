#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/alias.rs");
    t.pass("tests/default_derives.rs");
    t.compile_fail("tests/derives.rs");
    t.pass("tests/field_keys.rs");
    t.pass("tests/fields.rs");
    t.pass("tests/hashmap.rs");
    #[cfg(all(feature = "keys-serde", feature = "fields-serde"))]
    t.pass("tests/serde.rs");
}
