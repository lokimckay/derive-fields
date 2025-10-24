use crate::EnumKind;
use syn::{DeriveInput, Ident};

pub(crate) fn get_alias(input: &DeriveInput, kind: &EnumKind) -> Option<Ident> {
    let attr_name = match kind {
        EnumKind::Fields => "fields_alias",
        EnumKind::FieldKeys => "field_keys_alias",
    };

    input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(attr_name))
        .and_then(|attr| attr.parse_args::<Ident>().ok())
}
