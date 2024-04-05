extern crate proc_macro;

mod ast;
mod expand;
mod validate;

use std::{
    borrow::Borrow,
    cell::{Cell, Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    fmt::Error,
    hash::Hasher,
    rc::Rc,
    result,
};

use ast::{AstErrorDeclaration, AstErrorEnumVariant, AstErrorSet, AstErrorVariant, RefError};
use expand::{expand, ErrorEnum};
use proc_macro2::TokenStream;
use syn::Ident;
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

fn construct_error_enums(error_set: AstErrorSet) -> syn::Result<Vec<ErrorEnum>> {
    let mut error_enum_builders: Vec<ErrorEnumBuilder> = Vec::new();

    for declaration in error_set.set_items.into_iter() {
        let AstErrorDeclaration { error_name, parts } = declaration;

        let mut error_enum_builder = ErrorEnumBuilder::new(error_name);

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
    let error_enums = resolve(error_enum_builders.into_iter().map(RefCell::new).collect())?;

    Ok(error_enums)
}

fn resolve(error_enum_builders: Vec<RefCell<ErrorEnumBuilder>>) -> syn::Result<Vec<ErrorEnum>> {
    for index in 0..error_enum_builders.len() {
        if !error_enum_builders[index].borrow().ref_parts.is_empty() {
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
    error_enum_builders: &'a [RefCell<ErrorEnumBuilder>],
    visited: &mut Vec<Ident>,
) -> syn::Result<Vec<AstErrorEnumVariant>> {
    let this_error_enum_builder = &error_enum_builders[index];
    //println!("visited `{}`", visited.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" - "));
    if visited.contains(&this_error_enum_builder.borrow().error_name) {
        visited.push(this_error_enum_builder.borrow().error_name.clone());
        return Err(syn::parse::Error::new_spanned(
            this_error_enum_builder.borrow().error_name.clone(),
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
    let ref_parts_to_resolve = this_error_enum_builder
        .borrow()
        .ref_parts_to_resolve
        .clone();
    if !ref_parts_to_resolve.is_empty() {
        for ref_part in ref_parts_to_resolve {
            let ref_error_enum_index = error_enum_builders
                .iter()
                .position(|e| e.borrow().error_name == ref_part);
            let ref_error_enum_index = match ref_error_enum_index {
                Some(e) => e,
                None => {
                    return Err(syn::parse::Error::new_spanned(
                    &ref_part,
                    format!("error enum '{0}' includes error enum '{1}' as a subset, but '{1}' does not exist.", this_error_enum_builder.borrow().error_name, ref_part)
                ));
                }
            };
            let ref_error_enum_builder = &error_enum_builders[ref_error_enum_index];
            if !ref_error_enum_builder
                .borrow()
                .ref_parts_to_resolve
                .is_empty()
            {
                visited.push(this_error_enum_builder.borrow().error_name.clone());
                resolve_helper(ref_error_enum_index, error_enum_builders, visited)?;
                visited.pop();
            }
            for variant in ref_error_enum_builder.borrow().error_variants.iter() {
                let this_error_variants = &mut this_error_enum_builder.borrow_mut().error_variants;
                if !this_error_variants.contains(&variant) {
                    this_error_variants.push(variant.clone());
                }
            }
        }
        this_error_enum_builder
            .borrow_mut()
            .ref_parts_to_resolve
            .clear();
    }
    // Now that are refs are solved and included in this's error_variants, return them.
    Ok(this_error_enum_builder.borrow().error_variants.clone())
}

#[derive(Debug)]
struct ErrorEnumBuilder {
    pub error_name: Ident,
    pub error_variants: Vec<AstErrorEnumVariant>,
    pub ref_parts: Vec<RefError>,
    /// Once this is empty, all [ref_parts] have been resolved and [error_variants] is complete.
    pub ref_parts_to_resolve: Vec<RefError>,
}

impl ErrorEnumBuilder {
    fn new(error_name: Ident) -> Self {
        Self {
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
