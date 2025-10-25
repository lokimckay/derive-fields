use crate::{get_derives::get_derives, get_enum_name::get_enum_name, get_iter_impl::get_iter_impl};

use convert_case::Casing;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Type, parse_macro_input};

pub(crate) enum EnumKind {
    Fields,
    FieldKeys,
}

pub(crate) fn create_enum(input: TokenStream, kind: EnumKind) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = get_enum_name(&input, &kind);
    let derives = get_derives(&input, &kind);

    let fields = match input.data {
        Data::Struct(ref s) => match s.fields {
            Fields::Named(ref named) => named.named.iter().collect::<Vec<_>>(),
            _ => panic!("Only named fields supported"),
        },
        _ => panic!("Only structs supported"),
    };

    let variants: Vec<(Ident, Type)> = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            (
                syn::Ident::new(
                    &field_name.to_string().to_case(convert_case::Case::Pascal),
                    field_name.span(),
                ),
                f.ty.clone(),
            )
        })
        .collect();

    let variant_defs = variants.iter().map(|(ident, ty)| match kind {
        EnumKind::Fields => quote! { #ident(#ty) },
        EnumKind::FieldKeys => quote! { #ident },
    });

    let enum_def = quote! {
        #[derive( #( #derives ),* )]
        pub enum #enum_name {
            #( #variant_defs, )*
        }
    };

    let iter_impl = get_iter_impl(enum_name, &kind);

    quote! {
        #enum_def
        #iter_impl
    }
    .into()
}
