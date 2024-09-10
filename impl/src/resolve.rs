use crate::ast::{AstErrorDeclaration, AstErrorEnumVariant, AstErrorSet, RefError};
use crate::expand::ErrorEnum;

use syn::{Attribute, Ident};

/// Constructs [ErrorEnum]s from the ast, resolving any references to other sets. The returned result is
/// all error sets with the full expansion.
pub(crate) fn resolve(error_set: AstErrorSet) -> syn::Result<Vec<ErrorEnum>> {
    let mut error_enum_builders: Vec<ErrorEnumBuilder> = Vec::new();

    for declaration in error_set.set_items.into_iter() {
        let AstErrorDeclaration {
            attributes,
            error_name,
            parts,
        } = declaration;

        let mut error_enum_builder = ErrorEnumBuilder::new(error_name, attributes);

        for part in parts.into_iter() {
            match part {
                crate::ast::AstInlineOrRefError::Inline(inline_part) => {
                    error_enum_builder
                        .error_variants
                        .extend(inline_part.error_variants.into_iter());
                }
                crate::ast::AstInlineOrRefError::Ref(ref_part) => {
                    error_enum_builder.add_ref_part(ref_part);
                }
            }
        }
        error_enum_builders.push(error_enum_builder);
    }
    let error_enums = resolve_builders(error_enum_builders)?;

    Ok(error_enums)
}

fn resolve_builders(mut error_enum_builders: Vec<ErrorEnumBuilder>) -> syn::Result<Vec<ErrorEnum>> {
    for index in 0..error_enum_builders.len() {
        if !error_enum_builders[index].ref_parts_to_resolve.is_empty() {
            resolve_builders_helper(index, &mut *error_enum_builders, &mut Vec::new())?;
        }
    }
    let error_enums = error_enum_builders
        .into_iter()
        .map(Into::into)
        .collect::<Vec<ErrorEnum>>();
    Ok(error_enums)
}

fn resolve_builders_helper<'a>(
    index: usize,
    error_enum_builders: &'a mut [ErrorEnumBuilder],
    visited: &mut Vec<Ident>,
) -> syn::Result<Vec<AstErrorEnumVariant>> {
    //println!("visited `{}`", visited.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" - "));
    let error_enum_builder = &error_enum_builders[index];
    let error_name = &error_enum_builder.error_name;
    if visited.contains(error_name) {
        visited.push(error_name.clone());
        if let Some(pos) = visited.iter().position(|e| e == error_name) {
            visited.drain(0..pos);
        }
        return Err(syn::parse::Error::new_spanned(
            error_name.clone(),
            format!(
                "Cycle Detected: {}",
                visited
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("->")
            ),
        ));
    }
    let ref_parts_to_resolve = error_enum_builder.ref_parts_to_resolve.clone();
    // If this enums ref parts have not been resolved, resolve them.
    if !ref_parts_to_resolve.is_empty() {
        for ref_part in ref_parts_to_resolve {
            let ref_error_enum_index = error_enum_builders
                .iter()
                .position(|e| e.error_name == ref_part);
            let ref_error_enum_index = match ref_error_enum_index {
                Some(e) => e,
                None => {
                    return Err(syn::parse::Error::new_spanned(&ref_part, format!("Not a declared error set.")));
                }
            };
            if !error_enum_builders[ref_error_enum_index]
                .ref_parts_to_resolve
                .is_empty()
            {
                visited.push(error_enum_builders[index].error_name.clone());
                resolve_builders_helper(ref_error_enum_index, error_enum_builders, visited)?;
                visited.pop();
            }
            let (this_error_enum_builder, ref_error_enum_builder) =
                indices::indices!(&mut *error_enum_builders, index, ref_error_enum_index);
            for variant in ref_error_enum_builder.error_variants.iter() {
                let this_error_variants = &mut this_error_enum_builder.error_variants;
                if !this_error_variants.contains(&variant) {
                    this_error_variants.push(variant.clone());
                }
            }
        }
        error_enum_builders[index].ref_parts_to_resolve.clear();
    }
    // Now that are refs are solved and included in this error_enum_builder's error_variants, return them.
    Ok(error_enum_builders[index].error_variants.clone())
}


struct ErrorEnumBuilder {
    pub attributes: Vec<Attribute>,
    pub error_name: Ident,
    pub error_variants: Vec<AstErrorEnumVariant>,
    /// Once this is empty, all [ref_parts] have been resolved and [error_variants] is complete.
    pub ref_parts_to_resolve: Vec<RefError>,
}

impl ErrorEnumBuilder {
    fn new(error_name: Ident, attributes: Vec<Attribute>) -> Self {
        Self {
            attributes,
            error_name,
            error_variants: Vec::new(),
            ref_parts_to_resolve: Vec::new(),
        }
    }

    fn add_ref_part(&mut self, ref_part: RefError) {
        self.ref_parts_to_resolve.push(ref_part);
    }
}

impl From<ErrorEnumBuilder> for ErrorEnum {
    fn from(value: ErrorEnumBuilder) -> Self {
        assert!(
            value.ref_parts_to_resolve.is_empty(),
            "All references should be resolved when converting to an error enum."
        );
        ErrorEnum {
            attributes: value.attributes,
            error_name: value.error_name,
            error_variants: value.error_variants,
        }
    }
}

impl PartialEq for ErrorEnumBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.error_name == other.error_name
    }
}

impl std::hash::Hash for ErrorEnumBuilder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.error_name.hash(state);
    }
}

impl Eq for ErrorEnumBuilder {}