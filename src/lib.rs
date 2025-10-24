mod create_enum;
mod get_alias;
mod get_derives;

use create_enum::{EnumKind, create_enum};
use proc_macro::TokenStream;

#[proc_macro_derive(Fields, attributes(fields_derives, fields_alias))]
pub fn derive_keys_enum(input: TokenStream) -> TokenStream {
    create_enum(input, EnumKind::Fields)
}

#[proc_macro_derive(FieldKeys, attributes(field_keys_derives, field_keys_alias))]
pub fn derive_values_enum(input: TokenStream) -> TokenStream {
    create_enum(input, EnumKind::FieldKeys)
}
