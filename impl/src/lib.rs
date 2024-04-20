extern crate proc_macro;

mod ast;
mod expand;
mod validate;

use std::cell::Cell;

use ast::{AstErrorDeclaration, AstErrorEnumVariant, AstErrorSet, RefError};
use expand::{expand, ErrorEnum};

use syn::{Attribute, Ident};
use validate::validate;

#[proc_macro]
pub fn error_set(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let error_set = syn::parse_macro_input!(tokens as AstErrorSet);
    let error_enums = match construct_error_enums(error_set) {
        Ok(ok) => ok,
        Err(err) => {
            return err.into_compile_error().into();
        }
    };
    if let Err(err) = validate(&error_enums) {
        return err.into_compile_error().into();
    }
    expand(error_enums).into()
}

//************************************************************************//

fn construct_error_enums(error_set: AstErrorSet) -> syn::Result<Vec<ErrorEnum>> {
    let mut error_enum_builders: Vec<ErrorEnumBuilder> = Vec::new();

    for declaration in error_set.set_items.into_iter() {
        let AstErrorDeclaration { attributes: attribute, error_name, parts } = declaration;

        let mut error_enum_builder = ErrorEnumBuilder::new(error_name,attribute);

        for part in parts.into_iter() {
            match part {
                ast::AstInlineOrRefError::Inline(inline_part) => {
                    error_enum_builder
                        .error_variants
                        .extend(inline_part.error_variants.into_iter());
                }
                ast::AstInlineOrRefError::Ref(ref_part) => {
                    error_enum_builder.add_ref_part(ref_part);
                }
            }
        }
        error_enum_builders.push(error_enum_builder);
    }
    let error_enum_builders: Vec<Cell<ErrorEnumBuilder>> = unsafe {
        std::mem::transmute(error_enum_builders)
    };
    let error_enums = resolve(error_enum_builders)?;

    Ok(error_enums)
}

fn resolve(error_enum_builders: Vec<Cell<ErrorEnumBuilder>>) -> syn::Result<Vec<ErrorEnum>> {
    for index in 0..error_enum_builders.len() {
        let has_ref_parts;
        {
            let error_enum_builder = error_enum_builders[index].take();
            has_ref_parts = !error_enum_builder.ref_parts.is_empty();
            error_enum_builders[index].set(error_enum_builder);
        }
        if has_ref_parts {
            resolve_helper(index, &error_enum_builders, &mut Vec::new())?;
        }
    }
    let error_enums = error_enum_builders
        .into_iter()
        .map(|e| e.into_inner())
        .map(Into::into)
        .collect::<Vec<ErrorEnum>>();
    Ok(error_enums)
}

fn resolve_helper<'a>(
    index: usize,
    error_enum_builders: &'a [Cell<ErrorEnumBuilder>],
    visited: &mut Vec<Ident>,
) -> syn::Result<Vec<AstErrorEnumVariant>> {
    let this_error_enum_builder_cell = &error_enum_builders[index];
    //println!("visited `{}`", visited.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" - "));
    let mut error_enum_builder = this_error_enum_builder_cell.take();
    if visited.contains(&error_enum_builder.error_name) {
        visited.push(error_enum_builder.error_name.clone());
        return Err(syn::parse::Error::new_spanned(
            error_enum_builder.error_name.clone(),
            format!(
                "Recursive dependency: {}",
                visited
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("->")
            ),
        ));
    }
    let ref_parts_to_resolve = error_enum_builder
        .ref_parts_to_resolve
        .clone();
    if !ref_parts_to_resolve.is_empty() {
        for ref_part in ref_parts_to_resolve {
            let ref_error_enum_index = error_enum_builders
                .iter()
                .position(|e| {
                    let take = e.take();
                    let val  = take.error_name == ref_part;
                    e.set(take);
                    val
            });
            let ref_error_enum_index = match ref_error_enum_index {
                Some(e) => e,
                None => {
                    return Err(syn::parse::Error::new_spanned(
                    &ref_part,
                    format!("error enum '{0}' includes error enum '{1}' as a subset, but '{1}' does not exist.", error_enum_builder.error_name, ref_part)
                ));
                }
            };
            let ref_error_enum_builder_cell = &error_enum_builders[ref_error_enum_index];
            let mut ref_error_enum_builder = ref_error_enum_builder_cell.take();
            if !ref_error_enum_builder
                .ref_parts_to_resolve
                .is_empty()
            {
                visited.push(error_enum_builder.error_name.clone());
                ref_error_enum_builder_cell.set(ref_error_enum_builder);
                this_error_enum_builder_cell.set(error_enum_builder);
                resolve_helper(ref_error_enum_index, error_enum_builders, visited)?;
                ref_error_enum_builder = ref_error_enum_builder_cell.take();
                error_enum_builder = this_error_enum_builder_cell.take();
                visited.pop();
            }
            for variant in ref_error_enum_builder.error_variants.iter() {
                let this_error_variants = &mut error_enum_builder.error_variants;
                if !this_error_variants.contains(&variant) {
                    this_error_variants.push(variant.clone());
                }
            }
            ref_error_enum_builder_cell.set(ref_error_enum_builder);
        }
        error_enum_builder
            .ref_parts_to_resolve
            .clear();
    }
    let error_variants = error_enum_builder.error_variants.clone();
    this_error_enum_builder_cell.set(error_enum_builder);
    // Now that are refs are solved and included in this's error_variants, return them.
    Ok(error_variants)
}


impl Default for ErrorEnumBuilder {
    fn default() -> Self {
        Self {
            attributes: Vec::new(),
            error_name: Ident::new("Default", proc_macro2::Span::call_site()),
            error_variants: Vec::new(),
            ref_parts: Vec::new(),
            ref_parts_to_resolve: Vec::new(),
        }
    }
}

// #[derive(Debug)]
struct ErrorEnumBuilder {
    pub attributes: Vec<Attribute>,
    pub error_name: Ident,
    pub error_variants: Vec<AstErrorEnumVariant>,
    pub ref_parts: Vec<RefError>,
    /// Once this is empty, all [ref_parts] have been resolved and [error_variants] is complete.
    pub ref_parts_to_resolve: Vec<RefError>,
}

impl ErrorEnumBuilder {
    fn new(error_name: Ident, attributes: Vec<Attribute>) -> Self {
        Self {
            attributes,
            error_name,
            error_variants: Vec::new(),
            ref_parts: Vec::new(),
            ref_parts_to_resolve: Vec::new(),
        }
    }

    fn add_ref_part(&mut self, ref_part: RefError) {
        self.ref_parts.push(ref_part.clone());
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
