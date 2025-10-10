use std::collections::HashMap;

use crate::ast::{
    AstErrorEnumDeclaration, AstErrorVariant, AstInlineErrorVariantField, Disabled, RefError,
};
use crate::expand::{ErrorEnum, ErrorVariant, Named, SourceStruct, SourceTuple, Struct};

use quote::ToTokens;
use syn::{Attribute, Ident, TypeParam, Visibility};

/// Constructs [ErrorEnum]s from the ast, resolving any references to other sets. The returned result is
/// all error sets with the full expansion.
pub(crate) fn resolve(
    error_enum_decls: Vec<AstErrorEnumDeclaration>,
) -> syn::Result<Vec<ErrorEnum>> {
    let mut error_enum_builders: Vec<ErrorEnumBuilder> = Vec::new();

    for declaration in error_enum_decls.into_iter() {
        let AstErrorEnumDeclaration {
            attributes,
            vis, // todo propogate vis
            error_name,
            generics,
            disabled,
            parts,
        } = declaration;

        let mut error_enum_builder =
            ErrorEnumBuilder::new(error_name, attributes, vis, generics, disabled);

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
                .position(|e| e.error_name == ref_part.name);
            let ref_error_enum_index = match ref_error_enum_index {
                Some(e) => e,
                None => {
                    return Err(syn::parse::Error::new_spanned(
                        &ref_part.name,
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
            let [this_error_enum_builder, ref_error_enum_builder] = error_enum_builders
                .get_disjoint_mut([index, ref_error_enum_index])
                .unwrap();
            // Let the ref declaration override the original generic declaration name to avoid collisions - `.. || X<T> ..`
            if ref_part.generic_refs.len() != ref_error_enum_builder.generics.len() {
                Err(syn::parse::Error::new_spanned(
                    &ref_part.name,
                    format!(
                        "A reference to {} was declared with {} generic param(s), but the original definition takes {}.",
                        ref_part.name,
                        ref_part.generic_refs.len(),
                        ref_error_enum_builder.generics.len()
                    ),
                ))?;
            }
            let mut error_variants = Vec::new();
            let error_variants = if ref_part.generic_refs.is_empty() {
                &ref_error_enum_builder.error_variants
            } else {
                fn ident_to_type(ident: Ident) -> syn::Type {
                    let segment = syn::PathSegment {
                        ident,
                        arguments: syn::PathArguments::None, // No generic arguments
                    };
                    let path = syn::Path {
                        leading_colon: None,
                        segments: {
                            let mut punctuated = syn::punctuated::Punctuated::new();
                            punctuated.push(segment);
                            punctuated
                        },
                    };
                    let type_path = syn::TypePath { qself: None, path };
                    syn::Type::Path(type_path)
                }

                // rename the generics inside the variant fields to the new declared name - for `...= X<T> ..`, `T` in this case.
                let mut generic_type_to_new_generic_type = HashMap::<syn::Type, syn::Type>::new();
                let mut generic_type_to_new_generic_type_str = HashMap::<String, String>::new();
                let mut generic_type_str_to_regex = HashMap::<String, regex::Regex>::new();
                for (ref_part_generic, ref_error_enum_generic) in ref_part
                    .generic_refs
                    .iter()
                    .zip(ref_error_enum_builder.generics.iter())
                {
                    let old = ref_error_enum_generic.ident.to_string();
                    // e.g. For "X", matches "<X>", but not "<X" or "X>" or "X"
                    let generic_identification_pattern = format!(
                        r"(?P<before>[^\w\d]){}(?P<after>[^\w\d])",
                        regex::escape(&old)
                    );
                    let re = regex::Regex::new(&generic_identification_pattern).unwrap();
                    generic_type_str_to_regex.insert(old.clone(), re);
                    let new = ref_part_generic.to_string();
                    generic_type_to_new_generic_type_str.insert(old, new);
                    generic_type_to_new_generic_type.insert(
                        ident_to_type(ref_error_enum_generic.ident.clone()),
                        ident_to_type(ref_part_generic.clone()),
                    );
                }

                for error_variant in ref_error_enum_builder.error_variants.iter() {
                    let new_fields = if let Some(fields) = &error_variant.fields {
                        let mut new_fields = Vec::new();
                        for field in fields.iter() {
                            new_fields.push(replace_generics_in_fields(
                                field,
                                &generic_type_to_new_generic_type,
                                &generic_type_to_new_generic_type_str,
                                &generic_type_str_to_regex,
                            ));
                        }
                        Some(new_fields)
                    } else {
                        None
                    };
                    error_variants.push(AstErrorVariant {
                        attributes: error_variant.attributes.clone(),
                        cfg_attributes: error_variant.cfg_attributes.clone(),
                        display: error_variant.display.clone(),
                        name: error_variant.name.clone(),
                        fields: new_fields,
                        source_type: error_variant.source_type.clone(),
                        backtrace_type: error_variant.backtrace_type.clone(),
                    });
                }
                &error_variants
            };
            for variant in error_variants {
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

/// If the error definitions occupy the same space. Useful since if this space is already occupied e.g. ` X = A || B`
/// If `A` has a variant like `V1(std::io::Error)` and `B` `V1(std::io::Error)`.
pub(crate) fn does_occupy_the_same_space(this: &AstErrorVariant, other: &AstErrorVariant) -> bool {
    return this.name == other.name;
}

// fn merge_generics(this: &mut Generics, other: &Generics) {
//     let other_params = other.params.iter().collect::<Vec<_>>();
//     for other_param in other_params {
//         if !this.params.iter().any(|param| param == other_param) {
//             this.params.push(other_param.clone());
//         }
//     }
//     let other_where = other.where_clause.as_ref();
//     if let Some(other_where) = other_where {
//         if let Some(this_where) = &mut this.where_clause {
//             this_where.predicates.extend(other_where.predicates.clone());
//         } else {
//             this.where_clause = Some(other_where.clone());
//         }
//     }
// }

struct ErrorEnumBuilder {
    pub attributes: Vec<Attribute>,
    pub vis: Visibility,
    pub error_name: Ident,
    pub generics: Vec<TypeParam>,
    pub disabled: Disabled,
    pub error_variants: Vec<AstErrorVariant>,
    /// Once this is empty, all [ref_parts] have been resolved and [error_variants] is complete.
    pub ref_parts_to_resolve: Vec<RefError>,
}

impl ErrorEnumBuilder {
    fn new(
        error_name: Ident,
        attributes: Vec<Attribute>,
        vis: Visibility,
        generics: Vec<TypeParam>,
        disabled: Disabled,
    ) -> Self {
        Self {
            attributes,
            vis,
            error_name,
            generics,
            disabled,
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
            vis: value.vis,
            error_name: value.error_name,
            generics: value.generics,
            disabled: value.disabled,
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
        cfg_attributes,
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
                cfg_attributes,
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
                cfg_attributes,
                display,
                name,
                fields,
            });
        }
        // e.g. `Variant(std::io::Error)`
        (None, Some(source_type)) => {
            return ErrorVariant::SourceTuple(SourceTuple {
                attributes,
                cfg_attributes,
                display,
                name,
                source_type,
            });
        }
        // e.g. `Variant {}`
        (None, None) => {
            return ErrorVariant::Named(Named {
                attributes,
                cfg_attributes,
                display,
                name,
            });
        }
    }
}

//************************************************************************//

fn replace_generics_in_fields(
    field: &AstInlineErrorVariantField,
    old_to_new: &HashMap<syn::Type, syn::Type>,
    old_to_new_str: &HashMap<String, String>,
    old_to_match_regex: &HashMap<String, regex::Regex>,
) -> AstInlineErrorVariantField {
    if old_to_new.contains_key(&field.r#type) {
        let new_type = old_to_new.get(&field.r#type).unwrap().clone();
        return AstInlineErrorVariantField {
            name: field.name.clone(),
            r#type: new_type.clone(),
        };
    }
    // return field.clone();
    let field_type_str = field.r#type.to_token_stream().to_string();
    for (original_type, new_type) in old_to_new_str {
        let regex = &old_to_match_regex[original_type];
        let replaced = replace_part(&field_type_str, new_type, regex);
        if field_type_str != replaced {
            let new_type = syn::parse_str::<syn::Type>(&replaced)
                .expect("Failed to parse replaced type back into type");
            return AstInlineErrorVariantField {
                name: field.name.clone(),
                r#type: new_type.clone(),
            };
        }
    }
    return field.clone();
}

/// Assumes regex is `"(?P<before>[^\w\d]){}(?P<after>[^\w\d])"` as declared earlier
fn replace_part(input: &str, replacement: &str, re: &regex::Regex) -> String {
    re.replace_all(input, |caps: &regex::Captures| {
        // Reconstruct the matched segment with the replacement
        format!("{}{}{}", &caps["before"], replacement, &caps["after"])
    })
    .to_string()
}
