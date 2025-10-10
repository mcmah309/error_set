#![cfg_attr(not(feature = "dev"), allow(dead_code))]
#![cfg_attr(not(feature = "dev"), allow(unused_variables))]

use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::{TokenStreamExt, quote};
use syn::{Attribute, Ident, ItemStruct, Lit, PathArguments, TypeParam, TypePath, Visibility};

use crate::ast::{AstErrorStruct, AstInlineErrorVariantField, Disabled, DisplayAttribute};

/// Expand the [ErrorEnum]s into code.
pub(crate) fn expand(
    error_enums: Vec<ErrorEnum>,
    error_structs: Vec<AstErrorStruct>,
) -> TokenStream {
    let mut token_stream = TokenStream::new();
    let mut graph: Vec<ErrorEnumGraphNode> = error_enums
        .into_iter()
        .map(|e| ErrorEnumGraphNode::new(e))
        .collect();

    // build a graph of valid conversion `From`'s
    for building_index in 0..graph.len() {
        'next_enum: for checking_index in 0..graph.len() {
            if checking_index == building_index {
                continue;
            }

            let mut variant_mappings = Vec::new();
            'look_for_next_variant_match: for (checking_variant_index, checking_variant) in graph
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
                    if is_conversion_target(checking_variant, building_variant) {
                        variant_mappings.push((checking_variant_index, building_variant_index));
                        continue 'look_for_next_variant_match;
                    }
                }
                continue 'next_enum;
            }
            graph[building_index]
                .froms
                .push((checking_index, variant_mappings));
        }
    }

    for error_enum_node in graph.iter() {
        add_code_for_node(error_enum_node, &*graph, &mut token_stream);
    }
    for error_struct in error_structs {
        add_struct_error(error_struct, &mut token_stream);
    }
    token_stream
}

fn add_struct_error(error_struct: AstErrorStruct, token_stream: &mut TokenStream) {
    let AstErrorStruct { r#struct, display } = error_struct;
    let ItemStruct {
        attrs,
        vis,
        struct_token,
        ident: struct_name,
        generics,
        fields,
        semi_token,
    } = &r#struct;
    let (impl_generics, ty_generics, where_generics) = &generics.split_for_impl();
    let debug = quote! { #[derive(Debug)] };
    token_stream.append_all(quote! {
        #(#attrs)*
        #debug
        #vis #struct_token #struct_name #impl_generics #where_generics #fields
    });

    let mut has_source_field = false;
    for field in fields.iter() {
        if field
            .ident
            .as_ref()
            .is_some_and(|e| e.to_string() == "source")
        {
            has_source_field = true;
            break;
        }
    }
    if has_source_field {
        token_stream.append_all(quote! {
            #[allow(unused_qualifications)]
            impl #impl_generics core::error::Error for #struct_name #ty_generics {
                fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
                    Some(&self.source)
                }
            }
        });
    } else {
        token_stream.append_all(quote! {
            impl #impl_generics core::error::Error for #struct_name #ty_generics {

            }
        });
    }

    if let Some(display) = display {
        let field_names = fields.iter().filter_map(|e| e.ident.as_ref());
        let tokens = &display.tokens;
        let display_write_tokens: TokenStream;
        // e.g. `opaque` (no point in using this here but keeping functionality is consistent with enum variants)
        if is_opaque(tokens.clone()) {
            display_write_tokens = quote::quote! {
                write!(f, "{}", stringify!(#struct_name))
            };
        } else if let Some(string) = extract_string_if_str_literal(tokens.clone()) {
            // e.g. `"{}"`
            if is_format_str(&string) {
                display_write_tokens = quote::quote! {
                    write!(f, #tokens)
                };
            } else {
                // e.g. `"literal str"`
                display_write_tokens = quote::quote! {
                    write!(f, "{}", #tokens)
                };
            }
        } else {
            // e.g. `"field: {}", source.field`
            display_write_tokens = quote::quote! {
                write!(f, #tokens)
            };
        }
        token_stream.append_all(quote! {
            impl #impl_generics core::fmt::Display for #struct_name #ty_generics {
                #[allow(unused_qualifications)]
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    let #struct_name { #(#field_names),* } = &self;
                    #display_write_tokens
                }
            }
        });
    } else {
        token_stream.append_all(quote! {
            impl #impl_generics core::fmt::Display for #struct_name #ty_generics {
                #[inline]
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, "{}", stringify!(#struct_name))
                }
            }
        });
    }
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
        froms: _,
    } = error_enum_node;

    let enum_name = &error_enum.error_name;
    let error_variants = &error_enum.error_variants;
    #[cfg(feature = "dev")]
    assert!(
        !error_variants.is_empty(),
        "Error variants should not be empty"
    );
    let mut error_variant_tokens = TokenStream::new();
    for variant in error_variants {
        match variant {
            ErrorVariant::Named(named) => {
                let attributes = &named.attributes;
                let cfg_attributes = &named.cfg_attributes;
                let name = &named.name;
                error_variant_tokens.append_all(quote::quote! {
                    #(#cfg_attributes)*
                    #(#attributes)*
                    #name,
                });
            }
            ErrorVariant::Struct(r#struct) => {
                let attributes = &r#struct.attributes;
                let cfg_attributes = &r#struct.cfg_attributes;
                let name = &r#struct.name;
                let fields = &r#struct.fields;
                let field_names = fields.iter().map(|e| &e.name);
                let field_types = fields.iter().map(|e| &e.r#type);
                error_variant_tokens.append_all(quote::quote! {
                    #(#cfg_attributes)*
                    #(#attributes)*
                    #name {
                        #(#field_names : #field_types),*
                    },
                });
            }
            ErrorVariant::SourceStruct(source_struct) => {
                let attributes = &source_struct.attributes;
                let cfg_attributes = &source_struct.cfg_attributes;
                let name = &source_struct.name;
                let fields = &source_struct.fields;
                let field_names = fields.iter().map(|e| &e.name);
                let field_types = fields.iter().map(|e| &e.r#type);
                let source_type = &source_struct.source_type;
                error_variant_tokens.append_all(quote::quote! {
                    #(#cfg_attributes)*
                    #(#attributes)*
                    #name {
                        source: #source_type,
                        #(#field_names : #field_types),*
                    },
                });
            }
            ErrorVariant::SourceTuple(source_tuple) => {
                let attributes = &source_tuple.attributes;
                let cfg_attributes = &source_tuple.cfg_attributes;
                let name = &source_tuple.name;
                let source_type = &source_tuple.source_type;
                error_variant_tokens.append_all(quote::quote! {
                    #(#cfg_attributes)*
                    #(#attributes)*
                    #name(#source_type),
                });
            }
        }
    }
    let attributes = &error_enum.attributes;
    let vis = &error_enum.vis;
    let (impl_generics, ty_generics) = generic_tokens(&error_enum.generics);
    let debug = if error_enum.disabled.debug {
        quote! {}
    } else {
        quote! { #[derive(Debug)] }
    };
    token_stream.append_all(quote::quote! {
        #(#attributes)*
        #debug
        #vis enum #enum_name #impl_generics {
            #error_variant_tokens
        }
    });
}

fn impl_error(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    let ErrorEnumGraphNode {
        error_enum,
        froms: _,
    } = error_enum_node;
    if error_enum.disabled.error {
        return;
    }
    let enum_name = &error_enum.error_name;
    let mut source_match_branches = TokenStream::new();
    let mut has_source_match_branches = false;
    for variant in &error_enum.error_variants {
        if is_source_tuple_type(variant) {
            has_source_match_branches = true;
            let name = &variant.name();
            let cfg_attributes = &variant.cfg_attributes();
            source_match_branches.append_all(quote::quote! {
                #(#cfg_attributes)*
                #enum_name::#name(source) => source.source(),
            });
        } else if is_source_struct_type(variant) {
            has_source_match_branches = true;
            let name = &variant.name();
            let cfg_attributes = &variant.cfg_attributes();
            source_match_branches.append_all(quote::quote! {
                #(#cfg_attributes)*
                #enum_name::#name { source, .. } => source.source(),
            });
        }
    }
    let mut error_inner = TokenStream::new();
    if has_source_match_branches {
        error_inner.append_all(quote::quote! {
            fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
                match self {
                    #source_match_branches
                    #[allow(unreachable_patterns)]
                    _ => None,
                }
            }
        });
    }
    let (impl_generics, ty_generics) = generic_tokens(&error_enum.generics);
    token_stream.append_all(quote::quote! {
        #[allow(unused_qualifications)]
        impl #impl_generics core::error::Error for #enum_name #ty_generics {
            #error_inner
        }
    });
}

fn impl_display(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    let ErrorEnumGraphNode {
        error_enum,
        froms: _,
    } = error_enum_node;
    if error_enum.disabled.display {
        return;
    }
    let enum_name = &error_enum.error_name;
    let error_variants = &error_enum.error_variants;
    #[cfg(feature = "dev")]
    assert!(
        !error_variants.is_empty(),
        "Error variants should not be empty"
    );
    let mut error_variant_tokens = TokenStream::new();
    for variant in error_variants {
        let right_side: TokenStream;
        let name = &variant.name();
        if let Some(display) = &variant.display() {
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
            if is_source_tuple_type(variant) {
                right_side = quote::quote! {
                    write!(f, "{}", source)
                };
            } else {
                right_side = quote::quote! {
                    write!(f, "{}", concat!(stringify!(#enum_name), "::", stringify!(#name)))
                };
            }
        }

        match variant {
            ErrorVariant::Named(named) => {
                let cfg_attributes = &named.cfg_attributes;
                error_variant_tokens.append_all(quote::quote! {
                    #(#cfg_attributes)*
                    #enum_name::#name =>  #right_side,
                });
            }
            ErrorVariant::Struct(r#struct) => {
                let cfg_attributes = &r#struct.cfg_attributes;
                let field_names = r#struct.fields.iter().map(|e| &e.name);
                error_variant_tokens.append_all(quote::quote! {
                    #(#cfg_attributes)*
                    #enum_name::#name { #(#field_names),*  } =>  #right_side,
                });
            }
            ErrorVariant::SourceStruct(source_struct) => {
                let cfg_attributes = &source_struct.cfg_attributes;
                let field_names = source_struct.fields.iter().map(|e| &e.name);
                error_variant_tokens.append_all(quote::quote! {
                    #(#cfg_attributes)*
                    #enum_name::#name { source, #(#field_names),* } =>  #right_side,
                });
            }
            ErrorVariant::SourceTuple(source_tuple) => {
                let cfg_attributes = &source_tuple.cfg_attributes;
                error_variant_tokens.append_all(quote::quote! {
                    #(#cfg_attributes)*
                    #enum_name::#name(source) =>  #right_side,
                });
            }
        }
    }
    let (impl_generics, ty_generics) = generic_tokens(&error_enum.generics);
    token_stream.append_all(quote::quote! {
        impl #impl_generics core::fmt::Display for #enum_name #ty_generics {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match &*self {
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
    let from = &error_enum.disabled.from;
    if from.as_ref().is_some_and(|e| e.is_empty()) {
        return;
    }
    let temp = Vec::new();
    let froms_to_disable = from.as_ref().unwrap_or(&temp);
    let froms_to_disable_idents = froms_to_disable
        .iter()
        .flat_map(|e| e.path.get_ident())
        .collect::<Vec<_>>();
    let error_enum_name = &error_enum.error_name;

    for (from_error_enum, variant_mappings) in error_enum_node.resolved_froms(graph) {
        if froms_to_disable_idents.contains(&&from_error_enum.error_name) {
            continue;
        }
        let mut all_cfg_attributes = HashSet::new();
        let mut error_branch_tokens = TokenStream::new();
        let from_error_enum_name = &from_error_enum.error_name;
        for (from_error_enum_variant, error_enum_variant) in variant_mappings {
            #[cfg(feature = "dev")]
            {
                assert!(
                    from_error_enum
                        .error_variants
                        .iter()
                        .any(|e| e.name() == from_error_enum_variant.name()),
                    "Variant not found in from error enum"
                );
                assert!(
                    error_enum
                        .error_variants
                        .iter()
                        .any(|e| e.name() == error_enum_variant.name()),
                    "Variant not found in error enum"
                );
                let from = from_error_enum_variant.name();
                let to = error_enum_variant.name();
                assert!(
                    is_conversion_target(from_error_enum_variant, error_enum_variant),
                    "Not a valid conversion target\n\nfrom:\n\n{from}\n\nto:\n\n{to}"
                );
            }
            all_cfg_attributes.extend(from_error_enum_variant.cfg_attributes().clone());
            all_cfg_attributes.extend(error_enum_variant.cfg_attributes().clone());
            let arm: Option<TokenStream> = match (from_error_enum_variant, error_enum_variant) {
                (ErrorVariant::Named(this), ErrorVariant::Named(that)) => Some(name_to_name(
                    from_error_enum_name,
                    &this.name,
                    error_enum_name,
                    &that.name,
                )),
                (ErrorVariant::Named(this), ErrorVariant::Struct(that)) => None,
                (ErrorVariant::Named(this), ErrorVariant::SourceStruct(that)) => None,
                (ErrorVariant::Named(this), ErrorVariant::SourceTuple(that)) => None,
                (ErrorVariant::Struct(this), ErrorVariant::Named(that)) => None,
                (ErrorVariant::Struct(this), ErrorVariant::Struct(that)) => Some(struct_to_struct(
                    from_error_enum_name,
                    &this.name,
                    &this.fields,
                    error_enum_name,
                    &that.name,
                    &that.fields,
                )),
                (ErrorVariant::Struct(this), ErrorVariant::SourceStruct(that)) => None,
                (ErrorVariant::Struct(this), ErrorVariant::SourceTuple(that)) => None,
                (ErrorVariant::SourceStruct(this), ErrorVariant::Named(that)) => None,
                (ErrorVariant::SourceStruct(this), ErrorVariant::Struct(that)) => None,
                (ErrorVariant::SourceStruct(this), ErrorVariant::SourceStruct(that)) => {
                    Some(source_struct_to_source_struct(
                        from_error_enum_name,
                        &this.name,
                        &this.fields,
                        error_enum_name,
                        &that.name,
                        &that.fields,
                    ))
                }
                (ErrorVariant::SourceStruct(this), ErrorVariant::SourceTuple(that)) => {
                    Some(source_struct_to_source_tuple(
                        from_error_enum_name,
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
                            from_error_enum_name,
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
                        from_error_enum_name,
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
        // Dev Note: If from has generics and they are not the same as target's, then there is no guarantee that `impl_generics`
        // will contain all of and the correct generics definitions that are for `from_ty_generics`. Merging may cause
        // conflicts. This guard likely won't ever be removed since the correct mixture of generics may be
        // impossible to determine without the user explicitly specifying. Even if this guard does not hold,
        // an "unwanted" (but no compile error) `From` may be generated. This is an edge case and we are
        // being optimistic, so we don't just not implement `From` for all generics. But a user can opt-out
        // with `#[disable(From(..))]`
        if !from_error_enum.generics.is_empty() && error_enum.generics != from_error_enum.generics {
            continue;
        }
        let (impl_generics, ty_generics) = generic_tokens(&error_enum.generics);
        let (from_impl_generics, from_ty_generics) = generic_tokens(&from_error_enum.generics);
        let all_cfg_attributes = all_cfg_attributes.iter();
        token_stream.append_all(quote::quote! {
            #(#all_cfg_attributes)*
            impl #impl_generics From<#from_error_enum_name #from_ty_generics> for #error_enum_name #ty_generics {
                fn from(error: #from_error_enum_name #from_ty_generics) -> Self {
                    match error {
                        #error_branch_tokens
                    }
                }
            }
        });
    }

    // Do not impl `From` for source where source is the same between multiple variants
    let mut source_type_to_error_variants = HashMap::new();
    let mut all_source_types = HashSet::new();
    for error_variant in error_enum.error_variants.iter() {
        if let Some(source_type) = error_variant.source_type() {
            if froms_to_disable.contains(source_type) {
                continue;
            }
            if all_source_types.contains(source_type) {
                source_type_to_error_variants.remove(source_type);
            } else {
                all_source_types.insert(source_type);
                source_type_to_error_variants.insert(source_type, error_variant);
            }
        }
    }

    // Add `From`'s for all valid variants that are wrappers around source errors.
    for error_variant in source_type_to_error_variants.values() {
        let source_type = error_variant.source_type();
        let inner_source_type_if_concrete_box =
            source_type.and_then(|e| maybe_extract_known_wrapper_types(e));
        if is_source_tuple_type(error_variant) {
            let (impl_generics, ty_generics) = generic_tokens(&error_enum.generics);
            let variant_name = &error_variant.name();
            let cfg_attributes = &error_variant.cfg_attributes();
            token_stream.append_all(quote::quote! {
                #(#cfg_attributes)*
                impl #impl_generics From<#source_type> for #error_enum_name #ty_generics {
                    fn from(error: #source_type) -> Self {
                        #error_enum_name::#variant_name(error)
                    }
                }
            });
            if let Some(inner_known_wrapper_type) = inner_source_type_if_concrete_box {
                match inner_known_wrapper_type {
                    KnownWrapperTypes::Box(type_path) => {
                        token_stream.append_all(quote::quote! {
                            #(#cfg_attributes)*
                            impl #impl_generics From<#type_path> for #error_enum_name #ty_generics {
                                fn from(error: #type_path) -> Self {
                                    #error_enum_name::#variant_name(Box::new(error))
                                }
                            }
                        });
                    }
                    KnownWrapperTypes::TracedError(type_path) => {
                        token_stream.append_all(quote::quote! {
                            #(#cfg_attributes)*
                            impl #impl_generics From<#type_path> for #error_enum_name #ty_generics {
                                fn from(error: #type_path) -> Self {
                                    #error_enum_name::#variant_name(eros::TracedError::new(error))
                                }
                            }
                        });
                    }
                };
            }
        } else if is_source_only_struct_type(error_variant) {
            let (impl_generics, ty_generics) = generic_tokens(&error_enum.generics);
            let variant_name = &error_variant.name();
            let cfg_attributes = &error_variant.cfg_attributes();
            token_stream.append_all(quote::quote! {
                #(#cfg_attributes)*
                impl #impl_generics From<#source_type> for #error_enum_name #ty_generics {
                    fn from(error: #source_type) -> Self {
                        #error_enum_name::#variant_name { source: error }
                    }
                }
            });
            if let Some(inner_known_wrapper_type) = inner_source_type_if_concrete_box {
                match inner_known_wrapper_type {
                    KnownWrapperTypes::Box(type_path) => {
                        token_stream.append_all(quote::quote! {
                            #(#cfg_attributes)*
                            impl #impl_generics From<#type_path> for #error_enum_name #ty_generics {
                                fn from(error: #type_path) -> Self {
                                    #error_enum_name::#variant_name { source: Box::new(error) }
                                }
                            }
                        });
                    }
                    KnownWrapperTypes::TracedError(type_path) => {
                        token_stream.append_all(quote::quote! {
                            #(#cfg_attributes)*
                            impl #impl_generics From<#type_path> for #error_enum_name #ty_generics {
                                fn from(error: #type_path) -> Self {
                                    #error_enum_name::#variant_name { source: eros::TracedError::new(error) }
                                }
                            }
                        });
                    }
                };
            }
        }
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
    let this_field_names = this_enum_fields.iter().map(|e| &e.name);
    let that_field_names = that_enum_fields.iter().map(|e| &e.name);
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
    quote::quote! {
        #this_enum_name::#this_enum_variant_name { source, .. } =>  #that_enum_name::#that_enum_variant_name(source),
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
    let this_field_names = this_enum_fields.iter().map(|e| &e.name);
    let that_field_names = that_enum_fields.iter().map(|e| &e.name);
    quote::quote! {
        #this_enum_name::#this_enum_variant_name { source, #(#this_field_names),*  } =>  #that_enum_name::#that_variant_name { source, #(#that_field_names),* },
    }
}

pub(crate) trait Common {
    fn attributes(&self) -> &Vec<Attribute>;
    fn cfg_attributes(&self) -> &Vec<Attribute>;
    fn display(&self) -> Option<&DisplayAttribute>;
    fn name(&self) -> &Ident;
    fn fields(&self) -> Option<&Vec<AstInlineErrorVariantField>>;
    fn source_type(&self) -> Option<&syn::TypePath>;
}

#[derive(Clone)]
pub(crate) enum ErrorVariant {
    /// e.g. `ErrorVariantNamed,`
    Named(Named),
    /// e.g. `ErrorVariantNamed {...}`
    Struct(Struct),
    /// e.g. `ErrorVariantNamed(std::io::Error) {...}`
    SourceStruct(SourceStruct),
    /// e.g. `ErrorVariantNamed(std::io::Error)`
    SourceTuple(SourceTuple),
}

impl Common for ErrorVariant {
    fn attributes(&self) -> &Vec<Attribute> {
        match self {
            ErrorVariant::Named(e) => e.attributes(),
            ErrorVariant::Struct(e) => e.attributes(),
            ErrorVariant::SourceStruct(e) => e.attributes(),
            ErrorVariant::SourceTuple(e) => e.attributes(),
        }
    }
    fn cfg_attributes(&self) -> &Vec<Attribute> {
        match self {
            ErrorVariant::Named(e) => e.cfg_attributes(),
            ErrorVariant::Struct(e) => e.cfg_attributes(),
            ErrorVariant::SourceStruct(e) => e.cfg_attributes(),
            ErrorVariant::SourceTuple(e) => e.cfg_attributes(),
        }
    }
    fn display(&self) -> Option<&DisplayAttribute> {
        match self {
            ErrorVariant::Named(e) => e.display(),
            ErrorVariant::Struct(e) => e.display(),
            ErrorVariant::SourceStruct(e) => e.display(),
            ErrorVariant::SourceTuple(e) => e.display(),
        }
    }
    fn name(&self) -> &Ident {
        match self {
            ErrorVariant::Named(e) => e.name(),
            ErrorVariant::Struct(e) => e.name(),
            ErrorVariant::SourceStruct(e) => e.name(),
            ErrorVariant::SourceTuple(e) => e.name(),
        }
    }
    fn fields(&self) -> Option<&Vec<AstInlineErrorVariantField>> {
        match self {
            ErrorVariant::Named(e) => e.fields(),
            ErrorVariant::Struct(e) => e.fields(),
            ErrorVariant::SourceStruct(e) => e.fields(),
            ErrorVariant::SourceTuple(e) => e.fields(),
        }
    }
    fn source_type(&self) -> Option<&syn::TypePath> {
        match self {
            ErrorVariant::Named(e) => e.source_type(),
            ErrorVariant::Struct(e) => e.source_type(),
            ErrorVariant::SourceStruct(e) => e.source_type(),
            ErrorVariant::SourceTuple(e) => e.source_type(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Named {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) cfg_attributes: Vec<Attribute>,
    pub(crate) display: Option<DisplayAttribute>,
    pub(crate) name: Ident,
}

impl Common for Named {
    fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }
    fn cfg_attributes(&self) -> &Vec<Attribute> {
        &self.cfg_attributes
    }
    fn display(&self) -> Option<&DisplayAttribute> {
        self.display.as_ref()
    }
    fn name(&self) -> &Ident {
        &self.name
    }
    fn fields(&self) -> Option<&Vec<AstInlineErrorVariantField>> {
        None
    }
    fn source_type(&self) -> Option<&syn::TypePath> {
        None
    }
}

#[derive(Clone)]
pub(crate) struct Struct {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) cfg_attributes: Vec<Attribute>,
    pub(crate) display: Option<DisplayAttribute>,
    pub(crate) name: Ident,
    // Dev Note: This field will never be empty. Otherwise it should just be a [Named]
    pub(crate) fields: Vec<AstInlineErrorVariantField>,
}

impl Common for Struct {
    fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }
    fn cfg_attributes(&self) -> &Vec<Attribute> {
        &self.cfg_attributes
    }
    fn display(&self) -> Option<&DisplayAttribute> {
        self.display.as_ref()
    }
    fn name(&self) -> &Ident {
        &self.name
    }
    fn fields(&self) -> Option<&Vec<AstInlineErrorVariantField>> {
        Some(&self.fields)
    }
    fn source_type(&self) -> Option<&syn::TypePath> {
        None
    }
}

#[derive(Clone)]
pub(crate) struct SourceStruct {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) cfg_attributes: Vec<Attribute>,
    pub(crate) display: Option<DisplayAttribute>,
    pub(crate) name: Ident,
    pub(crate) source_type: syn::TypePath,
    // Dev Note: This field can be empty
    pub(crate) fields: Vec<AstInlineErrorVariantField>,
}

impl Common for SourceStruct {
    fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }
    fn cfg_attributes(&self) -> &Vec<Attribute> {
        &self.cfg_attributes
    }
    fn display(&self) -> Option<&DisplayAttribute> {
        self.display.as_ref()
    }
    fn name(&self) -> &Ident {
        &self.name
    }
    fn fields(&self) -> Option<&Vec<AstInlineErrorVariantField>> {
        Some(&self.fields)
    }
    fn source_type(&self) -> Option<&syn::TypePath> {
        Some(&self.source_type)
    }
}

#[derive(Clone)]
pub(crate) struct SourceTuple {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) cfg_attributes: Vec<Attribute>,
    pub(crate) display: Option<DisplayAttribute>,
    pub(crate) name: Ident,
    pub(crate) source_type: syn::TypePath,
}

impl Common for SourceTuple {
    fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }
    fn cfg_attributes(&self) -> &Vec<Attribute> {
        &self.cfg_attributes
    }
    fn display(&self) -> Option<&DisplayAttribute> {
        self.display.as_ref()
    }
    fn name(&self) -> &Ident {
        &self.name
    }
    fn fields(&self) -> Option<&Vec<AstInlineErrorVariantField>> {
        None
    }
    fn source_type(&self) -> Option<&syn::TypePath> {
        Some(&self.source_type)
    }
}

//************************************************************************//
#[derive(Clone)]
struct ErrorEnumGraphNode {
    pub(crate) error_enum: ErrorEnum,
    /// nodes where this error enum can be converted to the other error enum
    /// 0: index of target enum in graph
    /// 1: variant mapping
    ///   0: the from's error_variants's index
    ///   1: this's error_variants's index
    pub(crate) froms: Vec<(usize, Vec<(usize, usize)>)>,
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
            froms: Vec::new(),
        }
    }

    /// Returns an iterator of all the froms of this error enum. And the variant mappings from this to that.
    pub(crate) fn resolved_froms<'a>(
        &'a self,
        graph: &'a [ErrorEnumGraphNode],
    ) -> impl Iterator<Item = (&'a ErrorEnum, Vec<(&'a ErrorVariant, &'a ErrorVariant)>)> {
        self.froms.iter().map(|e| {
            let from = &graph[e.0];
            let variant_mappings =
                e.1.iter()
                    .map(|(from_index, this_index)| {
                        (
                            &from.error_enum.error_variants[*from_index],
                            &self.error_enum.error_variants[*this_index],
                        )
                    })
                    .collect::<Vec<_>>();
            (&from.error_enum, variant_mappings)
        })
    }
}

#[derive(Clone)]
pub(crate) struct ErrorEnum {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) vis: Visibility,
    pub(crate) error_name: Ident,
    pub(crate) generics: Vec<TypeParam>,
    pub(crate) disabled: Disabled,
    pub(crate) error_variants: Vec<ErrorVariant>,
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

fn generic_tokens(generics: &Vec<TypeParam>) -> (Option<TokenStream>, Option<TokenStream>) {
    if generics.is_empty() {
        return (None, None);
    }
    let impl_clause = quote! {<#(#generics),*>};

    let names = generics.iter().map(|e| &e.ident);
    let ty_clause = quote! {<#(#names),*>};

    (Some(impl_clause), Some(ty_clause))
}

//************************************************************************//

pub(crate) fn is_source_tuple_type(error_variant: &ErrorVariant) -> bool {
    return error_variant.source_type().is_some() && error_variant.fields().is_none();
}

pub(crate) fn is_source_only_struct_type(error_variant: &ErrorVariant) -> bool {
    return error_variant.source_type().is_some()
        && error_variant
            .fields()
            .as_ref()
            .is_some_and(|e| e.is_empty());
}

pub(crate) fn is_source_struct_type(error_variant: &ErrorVariant) -> bool {
    return error_variant.source_type().is_some() && error_variant.fields().as_ref().is_some();
}

/// To determine if [this] can be converted into [that] without dropping values.
/// Ignoring backtrace (since this is generated in the `From` impl if missing) and display.
/// This does not mean [this] is a subset of [that].
/// Why do they need to be exact?
/// e.g.
/// ```
/// X {
///   a: String,
///   b: u32,
/// }
/// ```
/// The above can be converted to the below, by droping the `b`. Even though the below could be considered a "subset".
/// ```
/// Y {
///   a: String
/// }
/// ```
/// If the below was also in the target enum, it would also be valid conversion target
/// ```
/// Z {
///  b: u32
/// }
/// ```
/// Thus, the names and shapes must be exactly the same to avoid this.
/// Note, there can multiple source tuples or sources only structs with the same wrapped error types (different names).
/// The first that is encountered becomes the `From` impl of that source error type.
/// To ensure the correct one is selected, pay attention to `X = A || B` ordering
/// or define your own `X = { IoError(std::io::Error) } || A || B`
///
/// Another example:
/// ```
///  N1 {
///     field: i32
///  }
/// ```
/// ==
/// ```
/// N1 {
///     field: i32
///  }
/// ```
/// !=
/// ```
/// N2 {
///     field: i32
///  }
/// ```
pub(crate) fn is_conversion_target(this: &ErrorVariant, that: &ErrorVariant) -> bool {
    return match (&this.source_type(), &that.source_type()) {
        (Some(this_source_type), Some(other_source_type)) => {
            this_source_type.path == other_source_type.path
                && this.name() == that.name()
                && this.fields() == that.fields()
        }
        (None, None) => this.name() == that.name() && this.fields() == that.fields(),
        _ => false,
    };
}

//************************************************************************//

fn maybe_extract_known_wrapper_types(ty: &TypePath) -> Option<KnownWrapperTypes<'_>> {
    let last_part = ty
        .path
        .segments
        .last()
        .expect("If segments exist there should be more than one.");
    let wrapper = match &*last_part.ident.to_string() {
        "Box" => KnownWrapperTypes::Box,
        "eros::TE" | "eros::TracedError" | "TE" | "TracedError" => KnownWrapperTypes::TracedError,
        _ => return None,
    };
    match &last_part.arguments {
        PathArguments::AngleBracketed(box_args) => {
            if box_args.args.len() != 1 {
                return None;
            }
            let box_arg = box_args.args.first().unwrap();
            match box_arg {
                syn::GenericArgument::Type(box_type) => match box_type {
                    syn::Type::Path(valid_box_type) => Some(wrapper(valid_box_type)),
                    _ => None,
                },
                _ => None,
            }
        }
        _ => None,
    }
}

enum KnownWrapperTypes<'a> {
    Box(&'a TypePath),
    TracedError(&'a TypePath),
}
