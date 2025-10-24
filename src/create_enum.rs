use crate::{get_derives::get_derives, get_enum_name::get_enum_name};

use convert_case::Casing;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

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

    let variants = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let variant_name = syn::Ident::new(
            &field_name.to_string().to_case(convert_case::Case::Pascal),
            field_name.span(),
        );
        let ty = &f.ty;

        match kind {
            EnumKind::Fields => quote! { #variant_name(#ty) },
            EnumKind::FieldKeys => quote! { #variant_name },
        }
    });

    let enum_def = quote! {
        #[derive( #( #derives ),* )]
        pub enum #enum_name {
            #( #variants, )*
        }
    };

    enum_def.into()
}
