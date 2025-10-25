use crate::EnumKind;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

#[cfg(feature = "iter")]
pub(crate) fn get_iter_impl(
    enum_name: Ident,
    variants: Vec<(Ident, Type)>,
    kind: &EnumKind,
) -> TokenStream {
    match kind {
        EnumKind::FieldKeys => {
            let variant_defs = variants
                .iter()
                .map(|(ident, _)| quote! { #enum_name::#ident });

            quote! {

                impl #enum_name {
                    pub fn iter() -> impl Iterator<Item = #enum_name> {
                        [ #( #variant_defs ),* ].into_iter()
                    }
                }
            }
        }
        EnumKind::Fields => quote! {},
    }
}

#[cfg(not(feature = "iter"))]
pub(crate) fn get_iter_impl(_: Ident, _: Vec<(Ident, Type)>, _: &EnumKind) -> TokenStream {
    quote! {}
}
