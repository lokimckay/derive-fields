use crate::EnumKind;
use syn::{DeriveInput, Path, Token, parse_quote, punctuated::Punctuated};

pub(crate) fn get_derives(input: &DeriveInput, kind: &EnumKind) -> Vec<Path> {
    let attr_name = match kind {
        EnumKind::Fields => "fields_derives",
        EnumKind::FieldKeys => "field_keys_derives",
    };

    let mut derives: Vec<Path> = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(attr_name))
        .and_then(|attr| {
            attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)
                .ok()
                .map(|punctuated| punctuated.into_iter().collect())
        })
        .unwrap_or_default();

    if derives.is_empty() {
        derives = match kind {
            EnumKind::Fields => vec![
                parse_quote!(Debug),
                parse_quote!(Clone),
                #[cfg(feature = "fields-serde")]
                parse_quote!(serde::Serialize),
                #[cfg(feature = "fields-serde")]
                parse_quote!(serde::Deserialize),
            ],
            EnumKind::FieldKeys => vec![
                parse_quote!(Debug),
                parse_quote!(Clone),
                parse_quote!(Copy),
                parse_quote!(PartialEq),
                parse_quote!(Eq),
                parse_quote!(std::hash::Hash),
                parse_quote!(strum::EnumIter),
                #[cfg(feature = "keys-serde")]
                parse_quote!(serde::Serialize),
                #[cfg(feature = "keys-serde")]
                parse_quote!(serde::Deserialize),
            ],
        }
    }

    derives
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_parses_correctly() {
        let input = parse_quote! {
            #[fields_derives(Debug, Clone, Copy)]
            struct MyStruct;
        };

        let derives = get_derives(&input, &EnumKind::Fields);
        let derive_names: Vec<String> = derives
            .iter()
            .map(|p| p.segments.last().unwrap().ident.to_string())
            .collect();

        assert_eq!(derive_names, vec!["Debug", "Clone", "Copy"]);
    }

    #[test]
    fn test_returns_empty_when_missing() {
        let input = parse_quote! {
            struct MyStruct;
        };

        let derives = get_derives(&input, &EnumKind::Fields);
        assert!(derives.is_empty());
    }

    #[test]
    fn test_ignores_other_attributes() {
        let input = parse_quote! {
            #[some_other_attr(Debug)]
            struct MyStruct;
        };

        let derives = get_derives(&input, &EnumKind::Fields);
        assert!(derives.is_empty());
    }
}
