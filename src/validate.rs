use std::collections::HashSet;

use syn::Ident;

use crate::{ast::AstErrorEnumVariant, expand::ErrorEnum};



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
    let mut unique_variants: HashSet<&Ident> = HashSet::new();
    let mut unique_sources: HashSet<String> = HashSet::new();
    for error_enum in error_enums {
        for variant  in &error_enum.error_variants {
            match variant {
                AstErrorEnumVariant::SourceErrorVariant(source_variant) => {
                    let source_variant_name = &source_variant.name;
                    if unique_variants.contains(source_variant_name) {
                        return Err(syn::parse::Error::new_spanned(
                            quote::quote!{source_variant},
                            &format!("A variant with name '{0}' already exists in error enum '{1}'", source_variant_name, error_enum.error_name),
                        ));
                    }
                    unique_variants.insert(source_variant_name);
                    let source_variant_source = source_variant.source.path.segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>().join("::");
                    if unique_sources.contains(&source_variant_source) {
                        return Err(syn::parse::Error::new_spanned(
                            &source_variant.source,
                            &format!("A variant with source '{0}' already exists in error enum '{1}'", source_variant_source, error_enum.error_name),
                        ));
                    }
                    unique_sources.insert(source_variant_source);
                }
                AstErrorEnumVariant::Variant(variant) => {
                    if unique_variants.contains(variant) {
                        return Err(syn::parse::Error::new_spanned(
                            quote::quote!{variant},
                            &format!("A variant with name '{0}' already exists in error enum '{1}'", variant, error_enum.error_name),
                        ));
                    }
                },
            }
        }
        unique_variants.clear();
        unique_sources.clear();
    }
    Ok(())
}