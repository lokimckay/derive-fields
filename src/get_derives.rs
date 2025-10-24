use syn::{DeriveInput, Token, punctuated::Punctuated};

/// Parse an attribute like `#[keys_enum_derives(Debug, Clone)]`
pub(crate) fn get_derives(input: &DeriveInput, attr_name: &str) -> Vec<syn::Path> {
    input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(attr_name))
        .and_then(|attr| {
            attr.parse_args_with(Punctuated::<syn::Path, Token![,]>::parse_terminated)
                .ok()
                .map(|punctuated| punctuated.into_iter().collect())
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_parses_correctly() {
        let input = parse_quote! {
            #[my_derives(Debug, Clone, Copy)]
            struct MyStruct;
        };

        let derives = get_derives(&input, "my_derives");
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

        let derives = get_derives(&input, "my_derives");
        assert!(derives.is_empty());
    }

    #[test]
    fn test_ignores_other_attributes() {
        let input = parse_quote! {
            #[some_other_attr(Debug)]
            struct MyStruct;
        };

        let derives = get_derives(&input, "my_derives");
        assert!(derives.is_empty());
    }
}
