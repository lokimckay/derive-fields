#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/fields.rs");
    t.pass("tests/field_keys.rs");
    t.pass("tests/hashmap.rs");
}
