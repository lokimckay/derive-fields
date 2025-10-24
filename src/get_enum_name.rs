use crate::EnumKind;
use syn::{DeriveInput, Ident};

pub(crate) fn get_enum_name(input: &DeriveInput, kind: &EnumKind) -> Ident {
    let struct_name = input.ident.clone();

    let attr_name = match kind {
        EnumKind::Fields => "fields_name",
        EnumKind::FieldKeys => "field_keys_name",
    };

    let override_name = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(attr_name))
        .and_then(|attr| attr.parse_args::<Ident>().ok());

    match override_name {
        Some(override_name) => override_name,
        None => Ident::new(
            &format!(
                "{}{}",
                struct_name,
                match &kind {
                    EnumKind::Fields => "Field",
                    EnumKind::FieldKeys => "FieldKey",
                }
            ),
            struct_name.span(),
        ),
    }
}
