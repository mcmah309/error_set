use std::collections::HashSet;

use syn::Ident;

use crate::ast::{AstErrorSet, AstErrorSetItem};



pub fn validate(error_set: &AstErrorSet) -> Result<(), syn::Error> {
    all_enums_have_unique_names(error_set)
}

fn all_enums_have_unique_names(error_set: &AstErrorSet) -> Result<(), syn::Error> {
    let AstErrorSet {
        set_name,
        set_items
    } = error_set;

    let mut error_enums = set_items.iter().filter_map(|error_set_item| {
        if let AstErrorSetItem::ErrorEnum(error_enum) = error_set_item {
            return Some(error_enum);
        }
        return None;
    }).collect::<Vec<_>>();

    let mut unique_names: HashSet<&Ident> = HashSet::new();
    for error_enum in error_enums {
        if unique_names.contains(&error_enum.error_name) {
            return Err(syn::parse::Error::new_spanned(
                quote::quote!{error_enum},
                &format!("'{0}' already exists as an error enum.", error_enum.error_name),
            ));
        }
        unique_names.insert(&error_enum.error_name);
    }
    if unique_names.contains(set_name){
        return Err(syn::parse::Error::new_spanned(
            quote::quote!{error_set},
            &format!("The error set cannot have the same name as an error enum."),
        ));
    }
    Ok(())
}