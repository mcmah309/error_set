extern crate proc_macro;

mod ast;
mod expand;
mod validate;

use ast::{AstErrorEnumVariant, AstErrorSet, AstErrorSetItem};
use expand::{expand, ErrorEnum};
use validate::validate;

#[proc_macro]
pub fn error_set(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let error_set = syn::parse_macro_input!(tokens as AstErrorSet);
    let error_enums = construct_error_enums(error_set);
    if let Err(err) = validate(&error_enums) {
        return err.into_compile_error().into();
    }
    expand(error_enums).into()
}

pub(crate) fn construct_error_enums(error_set: AstErrorSet) -> Vec<ErrorEnum> {
    let AstErrorSet {
        set_name: error_set_name,
        set_items: error_set_items,
    } = error_set;
    // if a set has no items, it is a variant, not a set item
    let mut all_variants = Vec::new();

    let mut error_enums: Vec<ErrorEnum> = Vec::new();
    for error_set_item in error_set_items.into_iter() {
        match error_set_item {
            AstErrorSetItem::SourceErrorVariant(variant) => {
                let variant = AstErrorEnumVariant::SourceErrorVariant(variant);
                if !all_variants.contains(&variant) {
                    all_variants.push(variant.into())
                }
            }
            AstErrorSetItem::ErrorEnum(error_enum) => {
                assert!(!error_enum.error_variants.is_empty(), "All error enums should have variants, otherwise they should be clasified as a variant");
                for error_variant in error_enum.error_variants.iter() {
                    if !all_variants.contains(error_variant) {
                        all_variants.push(error_variant.clone());
                    }
                }
                error_enums.push(error_enum.into());
            }
            AstErrorSetItem::Variant(variant) => {
                let variant = AstErrorEnumVariant::Variant(variant);
                if !all_variants.contains(&variant) {
                    all_variants.push(variant.into())
                }
            }
        }
    }
    // Add set level
    error_enums.push(ErrorEnum {
        error_name: error_set_name,
        error_variants: all_variants,
    });
    return error_enums;
}