use crate::EnumKind;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

#[cfg(feature = "iter")]
pub(crate) fn get_iter_impl(enum_name: Ident, kind: &EnumKind) -> TokenStream {
    match kind {
        EnumKind::FieldKeys => {
            quote! {
                impl #enum_name {
                    pub fn iter() -> impl Iterator<Item = Self> {
                        <#enum_name as strum::IntoEnumIterator>::iter()
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
