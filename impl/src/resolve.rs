use crate::ast::{AstErrorDeclaration, AstErrorSet, AstErrorVariant, RefError};
use crate::expand::{ErrorEnum, ErrorVariant, Named, SourceStruct, SourceTuple, Struct};

use syn::{Attribute, Generics, Ident};

/// Constructs [ErrorEnum]s from the ast, resolving any references to other sets. The returned result is
/// all error sets with the full expansion.
pub(crate) fn resolve(error_set: AstErrorSet) -> syn::Result<Vec<ErrorEnum>> {
    let mut error_enum_builders: Vec<ErrorEnumBuilder> = Vec::new();

    for declaration in error_set.set_items.into_iter() {
        let AstErrorDeclaration {
            attributes,
            error_name,
            generics,
            parts,
        } = declaration;

        let mut error_enum_builder = ErrorEnumBuilder::new(error_name, attributes, generics);

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
) -> syn::Result<Vec<AstErrorVariant>> {
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
                    return Err(syn::parse::Error::new_spanned(
                        &ref_part,
                        "Not a declared error set.",
                    ));
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
            match (
                &this_error_enum_builder.generics,
                &ref_error_enum_builder.generics,
            ) {
                (Some(this_generics), Some(that_generics)) => {
                    if this_generics != that_generics {
                        // Dev Note: Merging generics may cause collisions in a combined definitions, e.g. `T: Debug` and `T`.
                        // or unintended sparsity, e.g. `T` and `G` when one would rather just have `T`.
                        return Err(syn::parse::Error::new_spanned(
                            &ref_part,
                            "Aggregating multiple different generic errors is not supported. \
                        Instead redefine the error set with the desired generics and fields.",
                        ));
                    }
                }
                (None, None) => {}
                (None, Some(generics)) => {
                    this_error_enum_builder.generics = Some(generics.clone());
                }
                (Some(_), None) => {}
            };
            for variant in ref_error_enum_builder.error_variants.iter() {
                let this_error_variants = &mut this_error_enum_builder.error_variants;
                let is_variant_already_in_enum = this_error_variants
                    .iter()
                    .any(|e| does_occupy_the_same_space(e, &variant));
                if !is_variant_already_in_enum {
                    this_error_variants.push(variant.clone());
                }
            }
        }
        error_enum_builders[index].ref_parts_to_resolve.clear();
    }
    // Now that are refs are solved and included in this error_enum_builder's error_variants, return them.
    Ok(error_enum_builders[index].error_variants.clone())
}

/// If the error defintions occupy the same space. Useful since if this space is already occupied e.g. ` X = A || B`
/// If `A` has a variant like `V1(std::io::Error)` and `B` `V1(std::io::Error)`.
pub(crate) fn does_occupy_the_same_space(this: &AstErrorVariant, other: &AstErrorVariant) -> bool {
    return this.name == other.name;
}

struct ErrorEnumBuilder {
    pub attributes: Vec<Attribute>,
    pub error_name: Ident,
    pub generics: Option<Generics>,
    pub error_variants: Vec<AstErrorVariant>,
    /// Once this is empty, all [ref_parts] have been resolved and [error_variants] is complete.
    pub ref_parts_to_resolve: Vec<RefError>,
}

impl ErrorEnumBuilder {
    fn new(error_name: Ident, attributes: Vec<Attribute>, generics: Option<Generics>) -> Self {
        Self {
            attributes,
            error_name,
            generics,
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
            generics: value.generics,
            error_variants: value
                .error_variants
                .into_iter()
                .map(|v| reshape(v))
                .collect::<Vec<_>>(),
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

//************************************************************************//

fn reshape(this: AstErrorVariant) -> ErrorVariant {
    let AstErrorVariant {
        attributes,
        display,
        name,
        fields,
        source_type,
        backtrace_type: _,
    } = this;
    match (fields, source_type) {
        // e.g. `Variant(std::io::Error) {}` or `Variant(std::io::Error) {...}`
        (Some(fields), Some(source_type)) => {
            return ErrorVariant::SourceStruct(SourceStruct {
                attributes,
                display,
                name,
                source_type,
                fields,
            });
        }
        // e.g. `Variant(std::io::Error)`
        (Some(fields), None) => {
            return ErrorVariant::Struct(Struct {
                attributes,
                display,
                name,
                fields,
            });
        }
        // e.g. `Variant(std::io::Error)`
        (None, Some(source_type)) => {
            return ErrorVariant::SourceTuple(SourceTuple {
                attributes,
                display,
                name,
                source_type,
            });
        }
        // e.g. `Variant {}`
        (None, None) => {
            return ErrorVariant::Named(Named {
                attributes,
                display,
                name,
            });
        }
    }
}
