use std::collections::HashSet;

use syn::Ident;

use crate::expand::ErrorEnum;

/// Additional validation logic
pub fn validate(error_enums: &Vec<ErrorEnum>) -> Result<(), syn::Error> {
    all_enums_have_unique_names(error_enums)?;
    unique_variant_names_per_enum(error_enums)
}

fn all_enums_have_unique_names(error_enums: &Vec<ErrorEnum>) -> Result<(), syn::Error> {
    let mut unique_names: HashSet<&Ident> = HashSet::new();
    for error_enum in error_enums {
        if unique_names.contains(&error_enum.error_name) {
            return Err(syn::parse::Error::new_spanned(
                quote::quote! {error_enum},
                &format!(
                    "'{0}' already exists as an error enum.",
                    error_enum.error_name
                ),
            ));
        }
        unique_names.insert(&error_enum.error_name);
    }
    Ok(())
}

fn unique_variant_names_per_enum(
    error_enums: &Vec<ErrorEnum>,
) -> Result<(), syn::Error> {
    let mut unique_variant_names: HashSet<&Ident> = HashSet::new();
    for error_enum in error_enums {
        for variant in &error_enum.error_variants {
            let variant_name = &variant.name;
            if unique_variant_names.contains(variant_name) {
                return Err(syn::parse::Error::new_spanned(
                    quote::quote! {variant},
                    &format!(
                        "A variant with name '{0}' already exists in error enum '{1}'",
                        variant_name, error_enum.error_name
                    ),
                ));
            }
            unique_variant_names.insert(variant_name);
        }
        unique_variant_names.clear();
    }
    Ok(())
}
