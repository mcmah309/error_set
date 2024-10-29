use std::{iter::zip, usize};

#[cfg(feature = "coerce_macro")]
use coerce_macro::add_coerce_macro;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{token::Token, Attribute, Ident, Lit};

use crate::{
    ast::{AstErrorVariant, AstInlineErrorVariantField, DisplayAttribute},
    is_conversion_target, is_source_tuple_type,
};

/// Expand the [ErrorEnum]s into code.
pub(crate) fn expand(error_enums: Vec<ErrorEnum>) -> TokenStream {
    let mut token_stream = TokenStream::new();
    #[cfg(feature = "coerce_macro")]
    add_coerce_macro(&error_enums, &mut token_stream);
    let mut graph: Vec<ErrorEnumGraphNode> = error_enums
        .into_iter()
        .map(|e| ErrorEnumGraphNode::new(e))
        .collect();

    // build a graph of sub-sets and sets based on if all the variants of one are included in another
    for building_index in 0..graph.len() {
        'next_enum: for checking_index in 0..graph.len() {
            if checking_index == building_index {
                continue;
            }

            // let all_variants_in_checking_are_found_in_building =
            //     graph[checking_index]
            //         .error_enum
            //         .error_variants
            //         .iter()
            //         .all(|checking_variant| {
            //             graph[building_index].error_enum.error_variants.iter().any(
            //                 |building_variant| {
            //                     is_this_a_subset_of_that(checking_variant, building_variant)
            //                 },
            //             )
            //         });
            // if all_variants_in_checking_are_found_in_building {
            //     graph[building_index].subsets.push(checking_index);
            // }

            let mut variant_mappings = Vec::new();
            // Each checking variant must be a subset of at least one building variant. Else move on.
            'look_for_next_variant_subset: for (checking_variant_index, checking_variant) in graph
                [checking_index]
                .error_enum
                .error_variants
                .iter()
                .enumerate()
            {
                for (building_variant_index, building_variant) in graph[building_index]
                    .error_enum
                    .error_variants
                    .iter()
                    .enumerate()
                {
                    //panic!("check: {}\n\nbuil:{}", checking_variant.name, building_variant.name);
                    if is_conversion_target(checking_variant, building_variant) {
                        //panic!("{} is a subset of {}", checking_variant.name, building_variant.name);
                        variant_mappings.push((checking_variant_index, building_variant_index));
                        // subset found for checking_variant move on to check the next one.
                        continue 'look_for_next_variant_subset;
                    }
                }
                // subset not found for checking_variant. Since a variant is not in building_index it is not a subset.
                continue 'next_enum;
            }
            //panic!("subset: {:?}\n\nregular:{:?}", graph[checking_index].error_enum.error_name, graph[building_index].error_enum.error_name);
            //let mut building = &mut graph[building_index];
            graph[building_index]
                .subsets
                .push((checking_index, variant_mappings));
        }
    }

    for error_enum_node in graph.iter() {
        add_code_for_node(error_enum_node, &*graph, &mut token_stream);
    }
    token_stream
}

fn add_code_for_node(
    error_enum_node: &ErrorEnumGraphNode,
    graph: &[ErrorEnumGraphNode],
    token_stream: &mut TokenStream,
) {
    add_enum(error_enum_node, token_stream);
    impl_error(error_enum_node, token_stream);
    impl_display(error_enum_node, token_stream);
    impl_froms(error_enum_node, graph, token_stream);
}

fn add_enum(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    let ErrorEnumGraphNode {
        error_enum,
        subsets: _,
    } = error_enum_node;

    let enum_name = &error_enum.error_name;
    let error_variants = &error_enum.error_variants;
    assert!(
        !error_variants.is_empty(),
        "Error variants should not be empty"
    );
    let mut error_variant_tokens = TokenStream::new();
    for variant in error_variants {
        let variant = reshape(&variant);
        match variant {
            ErrorVariant::Named(named) => {
                let attributes = &named.attributes;
                let name = named.name;
                error_variant_tokens.append_all(quote::quote! {
                    #(#attributes)*
                    #name,
                });
            }
            ErrorVariant::Struct(r#struct) => {
                let attributes = &r#struct.attributes;
                let name = &r#struct.name;
                let fields = &r#struct.fields;
                let field_names = &fields.iter().map(|e| &e.name).collect::<Vec<_>>();
                let field_types = &fields.iter().map(|e| &e.r#type).collect::<Vec<_>>();
                error_variant_tokens.append_all(quote::quote! {
                    #(#attributes)*
                    #name {
                        #(#field_names : #field_types),*
                    },
                });
            }
            ErrorVariant::SourceStruct(source_struct) => {
                let attributes = &source_struct.attributes;
                let name = &source_struct.name;
                let fields = &source_struct.fields;
                let field_names = &fields.iter().map(|e| &e.name).collect::<Vec<_>>();
                let field_types = &fields.iter().map(|e| &e.r#type).collect::<Vec<_>>();
                let source_type = &source_struct.source_type;
                error_variant_tokens.append_all(quote::quote! {
                    #(#attributes)*
                    #name {
                        source: #source_type,
                        #(#field_names : #field_types),*
                    },
                });
            }
            ErrorVariant::SourceTuple(source_tuple) => {
                let attributes = &source_tuple.attributes;
                let name = &source_tuple.name;
                let source_type = &source_tuple.source_type;
                error_variant_tokens.append_all(quote::quote! {
                    #(#attributes)*
                    #name(#source_type),
                });
            }
        }
    }
    let attributes = &error_enum.attributes;
    token_stream.append_all(quote::quote! {
        #(#attributes)*
        #[derive(Debug)]
        pub enum #enum_name {
            #error_variant_tokens
        }
    });
}

fn impl_error(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    let ErrorEnumGraphNode {
        error_enum,
        subsets: _,
    } = error_enum_node;

    let enum_name = &error_enum.error_name;
    let mut source_match_branches = TokenStream::new();
    let mut has_source_match_branches = false;
    for variant in &error_enum.error_variants {
        if is_source_tuple_type(variant) {
            has_source_match_branches = true;
            let name = &variant.name;
            source_match_branches.append_all(quote::quote! {
                #enum_name::#name(ref source) => source.source(),
            });
        }
    }
    let mut error_inner = TokenStream::new();
    if has_source_match_branches {
        error_inner.append_all(quote::quote! {
            fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
                match *self {
                    #source_match_branches
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
        });
    }

    token_stream.append_all(quote::quote! {
        #[allow(unused_qualifications)]
        impl core::error::Error for #enum_name {
            #error_inner
        }
    });
}

fn impl_display(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    let ErrorEnumGraphNode {
        error_enum,
        subsets: _,
    } = error_enum_node;

    let enum_name = &error_enum.error_name;
    let error_variants = &error_enum.error_variants;
    assert!(
        !error_variants.is_empty(),
        "Error variants should not be empty"
    );
    let mut error_variant_tokens = TokenStream::new();
    for variant in error_variants {
        let right_side: TokenStream;
        let name = &variant.name;
        if let Some(display) = &variant.display {
            let tokens = &display.tokens;
            // e.g. `opaque`
            if is_opaque(tokens.clone()) {
                right_side = quote::quote! {
                    write!(f, "{}", concat!(stringify!(#enum_name), "::", stringify!(#name)))
                };
            } else if let Some(string) = extract_string_if_str_literal(tokens.clone()) {
                // e.g. `"{}"`
                if is_format_str(&string) {
                    if is_source_tuple_type(variant) {
                        right_side = quote::quote! {
                            write!(f, #tokens, source)
                        };
                    } else {
                        right_side = quote::quote! {
                            write!(f, #tokens)
                        };
                    }
                } else {
                    // e.g. `"literal str"`
                    right_side = quote::quote! {
                        write!(f, "{}", #tokens)
                    };
                }
            } else {
                // e.g. `"field: {}", source.field`
                right_side = quote::quote! {
                    write!(f, #tokens)
                };
            }
        } else {
            right_side = quote::quote! {
                write!(f, "{}", concat!(stringify!(#enum_name), "::", stringify!(#name)))
            };
        }

        let variant = reshape(variant);
        match variant {
            ErrorVariant::Named(_) => {
                error_variant_tokens.append_all(quote::quote! {
                    #enum_name::#name =>  #right_side,
                });
            }
            ErrorVariant::Struct(r#struct) => {
                let field_names = &r#struct.fields.iter().map(|e| &e.name).collect::<Vec<_>>();
                error_variant_tokens.append_all(quote::quote! {
                    #enum_name::#name { #(ref #field_names),*  } =>  #right_side,
                });
            }
            ErrorVariant::SourceStruct(source_struct) => {
                let field_names = &source_struct
                    .fields
                    .iter()
                    .map(|e| &e.name)
                    .collect::<Vec<_>>();
                error_variant_tokens.append_all(quote::quote! {
                    #enum_name::#name { ref source, #(ref #field_names),* } =>  #right_side,
                });
            }
            ErrorVariant::SourceTuple(source_tuple) => {
                error_variant_tokens.append_all(quote::quote! {
                    #enum_name::#name(ref source) =>  #right_side,
                });
            }
        }
    }
    token_stream.append_all(quote::quote! {
        impl core::fmt::Display for #enum_name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    #error_variant_tokens
                }
            }
        }
    });
}

fn impl_froms(
    error_enum_node: &ErrorEnumGraphNode,
    graph: &[ErrorEnumGraphNode],
    token_stream: &mut TokenStream,
) {
    let error_enum = &error_enum_node.error_enum;
    let error_enum_name = &error_enum.error_name;

    for (subset_error_enum, variant_mappings) in error_enum_node.resolved_subsets(graph) {
        let mut error_branch_tokens = TokenStream::new();
        //panic!("subset: {:?}\n\nregular:{:?}", subset_error_enum.error_name, error_enum.error_name);
        let subset_error_enum_name = &subset_error_enum.error_name;
        for (subset_error_enum_variant, error_enum_variant) in variant_mappings {
            assert!(subset_error_enum
                .error_variants
                .iter()
                .any(|e| e.name == subset_error_enum_variant.name));
            assert!(error_enum
                .error_variants
                .iter()
                .any(|e| e.name == error_enum_variant.name));
            assert!(
                is_conversion_target(subset_error_enum_variant, error_enum_variant),
                "Not subset\n\n{error_enum_variant:?}\n\nsubset: {subset_error_enum_variant:?}"
            ); // todo comment out
            let subset_error_variant_reshaped = &reshape(subset_error_enum_variant);
            let error_variant_reshaped = &reshape(error_enum_variant);
            let arm: Option<TokenStream> =
                match (subset_error_variant_reshaped, error_variant_reshaped) {
                    (ErrorVariant::Named(this), ErrorVariant::Named(that)) => Some(name_to_name(
                        subset_error_enum_name,
                        &this.name,
                        error_enum_name,
                        &that.name,
                    )),
                    (ErrorVariant::Named(this), ErrorVariant::Struct(that)) => None,
                    (ErrorVariant::Named(this), ErrorVariant::SourceStruct(that)) => None,
                    (ErrorVariant::Named(this), ErrorVariant::SourceTuple(that)) => None,
                    (ErrorVariant::Struct(this), ErrorVariant::Named(that)) => None,
                    (ErrorVariant::Struct(this), ErrorVariant::Struct(that)) => {
                        Some(struct_to_struct(
                            subset_error_enum_name,
                            &this.name,
                            &this.fields,
                            error_enum_name,
                            &that.name,
                            &that.fields,
                        ))
                    }
                    (ErrorVariant::Struct(this), ErrorVariant::SourceStruct(that)) => None,
                    (ErrorVariant::Struct(this), ErrorVariant::SourceTuple(that)) => None,
                    (ErrorVariant::SourceStruct(this), ErrorVariant::Named(that)) => None,
                    (ErrorVariant::SourceStruct(this), ErrorVariant::Struct(that)) => None,
                    (ErrorVariant::SourceStruct(this), ErrorVariant::SourceStruct(that)) => {
                        Some(source_struct_to_source_struct(
                            subset_error_enum_name,
                            &this.name,
                            &this.fields,
                            error_enum_name,
                            &that.name,
                            &that.fields,
                        ))
                    }
                    (ErrorVariant::SourceStruct(this), ErrorVariant::SourceTuple(that)) => {
                        Some(source_struct_to_source_tuple(
                            subset_error_enum_name,
                            &this.name,
                            &this.fields,
                            error_enum_name,
                            &that.name,
                        ))
                    }
                    (ErrorVariant::SourceTuple(this), ErrorVariant::Named(that)) => None,
                    (ErrorVariant::SourceTuple(this), ErrorVariant::Struct(that)) => None,
                    (ErrorVariant::SourceTuple(this), ErrorVariant::SourceStruct(that)) => {
                        if that.fields.is_empty() {
                            Some(source_tuple_to_source_only_struct(
                                subset_error_enum_name,
                                &this.name,
                                error_enum_name,
                                &that.name,
                            ))
                        } else {
                            None
                        }
                    }
                    (ErrorVariant::SourceTuple(this), ErrorVariant::SourceTuple(that)) => {
                        Some(source_tuple_to_source_tuple(
                            subset_error_enum_name,
                            &this.name,
                            error_enum_name,
                            &that.name,
                        ))
                    }
                };
            if let Some(arm) = arm {
                error_branch_tokens.append_all(arm);
            }
        }

        token_stream.append_all(quote::quote! {
            impl From<#subset_error_enum_name> for #error_enum_name {
                fn from(error: #subset_error_enum_name) -> Self {
                    match error {
                        #error_branch_tokens
                    }
                }
            }
        });
    }

    let mut source_errors_froms_already_implemented = Vec::new();
    // Add all `From`'s for variants that are wrappers around source errors.
    for source in error_enum.error_variants.iter().filter_map(|e| {
        if is_source_tuple_type(e) { //todo change to include inline structs as well
            return Some(e);
        }
        None
    }) {
        let source_type = &source.source_type;
        if source_errors_froms_already_implemented.contains(&source_type) {
            continue;
        }
        let variant_name = &source.name;
        token_stream.append_all(quote::quote! {
            impl From<#source_type> for #error_enum_name {
                fn from(error: #source_type) -> Self {
                    #error_enum_name::#variant_name(error)
                }
            }
        });
        source_errors_froms_already_implemented.push(source_type);
        break;
    }
}
//************************************************************************//

fn name_to_name(
    this_enum_name: &Ident,
    this_enum_variant_name: &Ident,
    that_enum_name: &Ident,
    that_enum_variant_name: &Ident,
) -> TokenStream {
    quote::quote! {
        #this_enum_name::#this_enum_variant_name =>  #that_enum_name::#that_enum_variant_name,
    }
}

fn struct_to_struct(
    this_enum_name: &Ident,
    this_variant_name: &Ident,
    this_enum_fields: &Vec<AstInlineErrorVariantField>,
    that_enum_name: &Ident,
    that_variant_name: &Ident,
    that_enum_fields: &Vec<AstInlineErrorVariantField>,
) -> TokenStream {
    let this_field_names = this_enum_fields.iter().map(|e| &e.name).collect::<Vec<_>>();
    let that_field_names = that_enum_fields.iter().map(|e| &e.name).collect::<Vec<_>>();
    quote::quote! {
        #this_enum_name::#this_variant_name { #(#this_field_names),*  } =>  #that_enum_name::#that_variant_name { #(#that_field_names),*  },
    }
}

fn source_tuple_to_source_tuple(
    this_enum_name: &Ident,
    this_enum_variant_name: &Ident,
    that_enum_name: &Ident,
    that_enum_variant_name: &Ident,
) -> TokenStream {
    quote::quote! {
        #this_enum_name::#this_enum_variant_name(source) =>  #that_enum_name::#that_enum_variant_name(source),
    }
}

fn source_tuple_to_source_only_struct(
    this_enum_name: &Ident,
    this_enum_variant_name: &Ident,
    that_enum_name: &Ident,
    that_enum_variant_name: &Ident,
) -> TokenStream {
    quote::quote! {
        #this_enum_name::#this_enum_variant_name(source) =>  #that_enum_name::#that_enum_variant_name { source },
    }
}

fn source_struct_to_source_tuple(
    this_enum_name: &Ident,
    this_enum_variant_name: &Ident,
    this_enum_fields: &Vec<AstInlineErrorVariantField>,
    that_enum_name: &Ident,
    that_enum_variant_name: &Ident,
) -> TokenStream {
    let this_field_names = this_enum_fields.iter().map(|e| &e.name).collect::<Vec<_>>();
    quote::quote! {
        #this_enum_name::#this_enum_variant_name { source, #(#this_field_names),* } =>  #that_enum_name::#that_enum_variant_name(source),
    }
}

fn source_struct_to_source_struct(
    this_enum_name: &Ident,
    this_enum_variant_name: &Ident,
    this_enum_fields: &Vec<AstInlineErrorVariantField>,
    that_enum_name: &Ident,
    that_variant_name: &Ident,
    that_enum_fields: &Vec<AstInlineErrorVariantField>,
) -> TokenStream {
    let this_field_names = this_enum_fields.iter().map(|e| &e.name).collect::<Vec<_>>();
    let that_field_names = that_enum_fields.iter().map(|e| &e.name).collect::<Vec<_>>();
    quote::quote! {
        #this_enum_name::#this_enum_variant_name { source, #(#this_field_names),*  } =>  #that_enum_name::#that_variant_name { source, #(#that_field_names),* },
    }
}

#[derive(Clone)]
pub(crate) enum ErrorVariant<'a> {
    /// e.g. `ErrorVariantNamed,`
    Named(Named<'a>),
    /// e.g. `ErrorVariantNamed {...}`
    Struct(Struct<'a>),
    /// e.g. `ErrorVariantNamed(std::io::Error) {...}`
    SourceStruct(SourceStruct<'a>),
    /// e.g. `ErrorVariantNamed(std::io::Error)`
    SourceTuple(SourceTuple<'a>),
}

#[derive(Clone)]
pub(crate) struct Named<'a> {
    pub(crate) attributes: &'a Vec<Attribute>,
    pub(crate) display: &'a Option<DisplayAttribute>,
    pub(crate) name: &'a Ident,
}

#[derive(Clone)]
pub(crate) struct Struct<'a> {
    pub(crate) attributes: &'a Vec<Attribute>,
    pub(crate) display: &'a Option<DisplayAttribute>,
    pub(crate) name: &'a Ident,
    // Dev Note: This field will never be empty. Otherwise it should just be a [Named]
    pub(crate) fields: &'a Vec<AstInlineErrorVariantField>,
}

#[derive(Clone)]
pub(crate) struct SourceStruct<'a> {
    pub(crate) attributes: &'a Vec<Attribute>,
    pub(crate) display: &'a Option<DisplayAttribute>,
    pub(crate) name: &'a Ident,
    pub(crate) source_type: &'a syn::TypePath,
    // Dev Note: This field can be empty
    pub(crate) fields: &'a Vec<AstInlineErrorVariantField>,
}

#[derive(Clone)]
pub(crate) struct SourceOnlyStruct<'a> {
    pub(crate) attributes: &'a Vec<Attribute>,
    pub(crate) display: &'a Option<DisplayAttribute>,
    pub(crate) name: &'a Ident,
    pub(crate) source_type: &'a syn::TypePath,
}

#[derive(Clone)]
pub(crate) struct SourceTuple<'a> {
    pub(crate) attributes: &'a Vec<Attribute>,
    pub(crate) display: &'a Option<DisplayAttribute>,
    pub(crate) name: &'a Ident,
    pub(crate) source_type: &'a syn::TypePath,
}

fn reshape(this: &AstErrorVariant) -> ErrorVariant {
    let AstErrorVariant {
        attributes,
        display,
        name,
        fields,
        source_type,
        backtrace_type,
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

//************************************************************************//
#[derive(Clone)]
struct ErrorEnumGraphNode {
    pub(crate) error_enum: ErrorEnum,
    /// nodes where all error variants of the error enum are in this error enum's error variants.
    /// 0: index of subset enum in graph
    /// 1: variant mapping
    ///   0: the subset's error_variants's index
    ///   1: this [error_enum.error_variants]'s index
    pub(crate) subsets: Vec<(usize, Vec<(usize, usize)>)>,
}

impl PartialEq for ErrorEnumGraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.error_enum == other.error_enum
    }
}

impl ErrorEnumGraphNode {
    pub(crate) fn new(node: ErrorEnum) -> ErrorEnumGraphNode {
        ErrorEnumGraphNode {
            error_enum: node,
            subsets: Vec::new(),
        }
    }

    /// Returns an iterator of all the subsets of this error enum. And the variant mappings from this to that.
    pub(crate) fn resolved_subsets<'a>(
        &'a self,
        graph: &'a [ErrorEnumGraphNode],
    ) -> impl Iterator<
        Item = (
            &'a ErrorEnum,
            Vec<(&'a AstErrorVariant, &'a AstErrorVariant)>,
        ),
    > {
        self.subsets.iter().map(|e| {
            let subset = &graph[e.0];
            let variant_mappings =
                e.1.iter()
                    .map(|(subset_index, this_index)| {
                        (
                            &subset.error_enum.error_variants[*subset_index],
                            &self.error_enum.error_variants[*this_index],
                        )
                    })
                    .collect::<Vec<_>>();
            (&subset.error_enum, variant_mappings)
        })
    }
}

#[derive(Clone)]
pub(crate) struct ErrorEnum {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) error_name: Ident,
    pub(crate) error_variants: Vec<AstErrorVariant>,
}

impl core::hash::Hash for ErrorEnum {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.error_name.hash(state);
    }
}

impl Eq for ErrorEnum {}

impl PartialEq for ErrorEnum {
    fn eq(&self, other: &Self) -> bool {
        self.error_name == other.error_name
    }
}

//************************************************************************//

fn is_str_literal(input: TokenStream) -> bool {
    if let Ok(expr) = syn::parse2::<Lit>(input) {
        if let Lit::Str(_) = expr {
            return true;
        }
    }
    false
}

fn extract_string_if_str_literal(input: TokenStream) -> Option<String> {
    if let Ok(expr) = syn::parse2::<Lit>(input) {
        if let Lit::Str(lit) = expr {
            return Some(lit.value());
        }
    }
    None
}

// Dev Note: naive implementation.
fn is_format_str(input: &str) -> bool {
    let mut interpolation_candidate_found = false;
    let mut last_char = 'a';

    let mut start_count = 0;
    let mut end_count = 0;

    for c in input.chars() {
        if c == '{' {
            if last_char == '{' {
                last_char = 'a';
                start_count -= 1;
                continue;
            }
            start_count += 1;
        } else if c == '}' {
            if last_char == '}' {
                last_char = 'a';
                end_count -= 1;
                continue;
            }
            end_count += 1;
            if start_count == end_count {
                interpolation_candidate_found = true;
            }
        }
        last_char = c;
    }
    return interpolation_candidate_found && start_count == end_count;
}

fn is_opaque(input: TokenStream) -> bool {
    if let Ok(ident) = syn::parse2::<Ident>(input) {
        ident == "opaque"
    } else {
        false
    }
}

//************************************************************************//

#[cfg(feature = "coerce_macro")]
mod coerce_macro {
    //! ## The `coerce!` Macro
    //!
    //! The `coerce!` macro handles coercing between intersecting sets (sets where some of the error types are in common). This allows only being explicit where relevant, such as the disjointedness.
    //!
    //! e.g. given:
    //!
    //! ```rust
    //! error_set! {
    //!    SetX = {
    //!        X
    //!    } || Common;
    //!    SetY = {
    //!        Y
    //!    } || Common;
    //!    Common = {
    //!        A,
    //!        B,
    //!        C,
    //!        D,
    //!        E,
    //!        F,
    //!        G,
    //!        H,
    //!    };
    //! }
    //! ```
    //!
    //! rather than writing:
    //!
    //! ```rust
    //! fn setx_result_to_sety_result() -> Result<(), SetY> {
    //!    let _ok = match setx_result() {
    //!        Ok(ok) => ok,
    //!        Err(SetX::X) => {} // handle disjointedness
    //!        Err(SetX::A) => {
    //!            return Err(SetY::A);
    //!        }
    //!        Err(SetX::B) => {
    //!            return Err(SetY::B);
    //!        }
    //!        Err(SetX::C) => {
    //!            return Err(SetY::C);
    //!        }
    //!        Err(SetX::D) => {
    //!            return Err(SetY::D);
    //!        }
    //!        Err(SetX::E) => {
    //!            return Err(SetY::E);
    //!        }
    //!        Err(SetX::F) => {
    //!            return Err(SetY::F);
    //!        }
    //!        Err(SetX::G) => {
    //!            return Err(SetY::G);
    //!        }
    //!        Err(SetX::H) => {
    //!            return Err(SetY::H);
    //!        }
    //!    };
    //!    Ok(())
    //! }
    //! ```
    //!
    //! one can write this, which compiles to the `match` statement above:
    //!
    //! ```rust
    //! fn setx_result_to_sety_result() -> Result<(), SetY> {
    //!    let _ok = coerce!(setx_result() => {
    //!        Ok(ok) => ok,
    //!        Err(SetX::X) => {}, // handle disjointedness
    //!        { Err(SetX) => return Err(SetY) } // terminal coercion
    //!    });
    //!    Ok(())
    //! }
    //! ```
    //!
    //! The `coerce!` macro is a flat fast (no tt muncher 🦫) declarative macro created by the `error_set!` macro for the set.
    //! `coerce!` behaves like a regular `match` statement, except it allows a terminal coercion statement between sets. e.g.
    //!
    //! ```rust
    //! { Err(SetX) => return Err(SetY) }
    //! { Err(SetX) => Err(SetY) }
    //! { SetX => return SetY }
    //! { SetX => SetY }
    //! ```
    //!
    //! With `coerce!`, one can concisely handle specific variants of errors as they bubble up the call stack and propagate the rest.

    use proc_macro2::TokenStream;
    use quote::TokenStreamExt;
    use syn::Ident;

    use crate::ast::AstErrorEnumVariant;

    use super::ErrorEnum;

    pub(crate) fn add_coerce_macro(error_enums: &Vec<ErrorEnum>, token_stream: &mut TokenStream) {
        let enum_intersections: Vec<EnumIntersection> = construct_set_intersections(&error_enums);
        let mut macro_pattern_token_stream = TokenStream::new();
        for enum_interscetion in enum_intersections {
            let EnumIntersection {
                enum1: enum1_name,
                enum2: enum2_name,
                intersection,
            } = enum_interscetion;
            let mut match_arms_return_err = TokenStream::new();
            let mut match_arms_err = TokenStream::new();
            let mut match_arms_return = TokenStream::new();
            let mut match_arms = TokenStream::new();
            for variant in intersection {
                match variant {
                    AstErrorEnumVariant::WrappedVariant(source_variant) => {
                        let variant = source_variant.name;
                        match_arms_return_err.append_all(quote::quote! {
                        Err(#enum1_name::#variant(source)) => { return Err(#enum2_name::#variant(source)); },
                    });
                        match_arms_err.append_all(quote::quote! {
                            Err(#enum1_name::#variant(source)) => { Err(#enum2_name::#variant(source)) },
                        });
                        match_arms_return.append_all(quote::quote! {
                            #enum1_name::#variant(source) => { return #enum2_name::#variant(source); },
                        });
                        match_arms.append_all(quote::quote! {
                            #enum1_name::#variant(source) => { #enum2_name::#variant(source) },
                        });
                    }
                    AstErrorEnumVariant::InlineVariant(variant) => {
                        let name = variant.name;
                        let fields = variant.fields;
                        if fields.is_empty() {
                            match_arms_return_err.append_all(quote::quote! {
                                Err(#enum1_name::#name) => { return Err(#enum2_name::#name); },
                            });
                            match_arms_err.append_all(quote::quote! {
                                Err(#enum1_name::#name) => { Err(#enum2_name::#name) },
                            });
                            match_arms_return.append_all(quote::quote! {
                                #enum1_name::#name => { return #enum2_name::#name; },
                            });
                            match_arms.append_all(quote::quote! {
                                #enum1_name::#name => { #enum2_name::#name },
                            });
                        } else {
                            let field_names = fields.iter().map(|e| &e.name).collect::<Vec<_>>();
                            match_arms_return_err.append_all(quote::quote! {
                                Err(#enum1_name::#name { #(#field_names),*  }) => { return Err(#enum2_name::#name { #(#field_names),*  }); },
                            });
                            match_arms_err.append_all(quote::quote! {
                                Err(#enum1_name::#name { #(#field_names),*  }) => { Err(#enum2_name::#name { #(#field_names),*  }) },
                            });
                            match_arms_return.append_all(quote::quote! {
                                #enum1_name::#name { #(#field_names),*  } => { return #enum2_name::#name { #(#field_names),*  }; },
                            });
                            match_arms.append_all(quote::quote! {
                                #enum1_name::#name { #(#field_names),*  } => { #enum2_name::#name { #(#field_names),*  } },
                            });
                        }
                    }
                }
            }
            macro_pattern_token_stream.append_all(quote::quote! {
                ($expr:expr, $($patterns:pat => $results:expr$(,)?)+, {Err(#enum1_name) => return Err(#enum2_name)}) => {
                    match $expr {
                        $($patterns => $results,)+
                        #match_arms_return_err
                    }
                };
                ($expr:expr, $($patterns:pat => $results:expr$(,)?)+, {Err(#enum1_name) => Err(#enum2_name)}) => {
                    match $expr {
                        $($patterns => $results,)+
                        #match_arms_err
                    }
                };
                ($expr:expr, $($patterns:pat => $results:expr$(,)?)+, {#enum1_name => return #enum2_name}) => {
                    match $expr {
                        $($patterns => $results,)+
                        #match_arms_return
                    }
                };
                ($expr:expr, $($patterns:pat => $results:expr$(,)?)+, {#enum1_name => #enum2_name}) => {
                    match $expr {
                        $($patterns => $results,)+
                        #match_arms
                    }
                };
        });
        }
        // when no default coercion
        macro_pattern_token_stream.append_all(quote::quote! {
            ($expr:expr => { $($patterns:pat => $results:expr$(,)?)+ }) => {
                match $expr {
                    $($patterns => $results,)+
                }
            };
        });
        //
        macro_pattern_token_stream.append_all(quote::quote! {
            ($($other:tt)*) => {
                compile_error!(r#"
No patterns matched. 
Possible reasons:
    1. There are no intersections between the sets.
    3. The pattern is incorrect.
`coerce` is expected to follow the pattern:
```
coerce!($VAR => {
    <$arms>+
    <,{ one_of<
            <$FROM => $TO>,
            <$FROM => return $TO>,
            <Err($FROM) => Err($TO)>,
            <Err($FROM) => return Err($TO)>
        > } >?
});
```
"#)
            };
        });
        token_stream.append_all(quote::quote! {
            #[allow(unused_macros)]
            macro_rules! coerce {
                #macro_pattern_token_stream
            }

            pub(crate) use coerce;
        });
    }

    fn construct_set_intersections(error_enums: &Vec<ErrorEnum>) -> Vec<EnumIntersection> {
        let mut enum_intersections: Vec<EnumIntersection> = Vec::new();
        let length = error_enums.len();
        for index1 in 0..length {
            for index2 in 0..length {
                let enum1 = &error_enums[index1];
                let enum2 = &error_enums[index2];
                let mut intersections = Vec::new();
                for variant in &enum1.error_variants {
                    if enum2.error_variants.contains(&variant) {
                        intersections.push(variant.clone());
                    }
                }
                if !intersections.is_empty() {
                    let enum_intersection = EnumIntersection::new(
                        enum1.error_name.clone(),
                        enum2.error_name.clone(),
                        intersections,
                    );
                    enum_intersections.push(enum_intersection);
                }
            }
        }
        enum_intersections
    }

    struct EnumIntersection {
        pub(crate) enum1: Ident,
        pub(crate) enum2: Ident,
        pub(crate) intersection: Vec<AstErrorEnumVariant>,
    }

    impl EnumIntersection {
        pub(crate) fn new(
            enum1: Ident,
            enum2: Ident,
            intersection: Vec<AstErrorEnumVariant>,
        ) -> EnumIntersection {
            EnumIntersection {
                enum1,
                enum2,
                intersection,
            }
        }
    }
}
