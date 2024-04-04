use std::collections::HashSet;

use syn::Ident;

use crate::{ast::{AstErrorEnumVariant, AstErrorSet, AstErrorSetItem}, expand::ErrorEnum};



pub fn validate(error_enums: &Vec<ErrorEnum>) -> Result<(), syn::Error> {
    all_enums_have_unique_names(error_enums)?;
    only_one_source_of_each_type_per_enum(error_enums)
}

fn all_enums_have_unique_names(error_enums: &Vec<ErrorEnum>) -> Result<(), syn::Error> {
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
    Ok(())
}

fn only_one_source_of_each_type_per_enum(error_enums: &Vec<ErrorEnum>) -> Result<(), syn::Error> {
    Ok(())
}