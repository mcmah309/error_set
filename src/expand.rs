use std::{cell::RefCell, rc::Rc};

use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::Ident;

use crate::ast::{
    is_type_path_equal, AstErrorEnumVariant,
};

pub(crate) fn expand(error_enums: Vec<ErrorEnum>) -> TokenStream {
    let error_enum_nodes: Vec<Rc<RefCell<ErrorEnumGraphNode>>> = error_enums
        .into_iter()
        .map(|e| Rc::new(RefCell::new(ErrorEnumGraphNode::new(e.into()))))
        .collect();
    for building_node in error_enum_nodes.iter() {
        for checking_node in error_enum_nodes.iter() {
            if (*(**checking_node).borrow()).error_enum != (*(**building_node).borrow()).error_enum
                && (*(**checking_node).borrow())
                    .error_enum
                    .error_variants
                    .iter()
                    .all(|e| {
                        (*(**building_node).borrow())
                            .error_enum
                            .error_variants
                            .contains(e)
                    })
            {
                building_node
                    .borrow_mut()
                    .subsets
                    .push(checking_node.clone());
            }
        }
    }

    let mut token_stream = TokenStream::new();
    for error_enum_node in error_enum_nodes.iter() {
        add_code_for_node(&*(**error_enum_node).borrow(), &mut token_stream);
    }
    token_stream
}

fn add_code_for_node(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    add_enum(error_enum_node, token_stream);
    impl_error(error_enum_node, token_stream);
    impl_display_and_debug(error_enum_node, token_stream);
    impl_froms(error_enum_node, token_stream);
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
    for error_variant in error_variants {
        match error_variant {
            AstErrorEnumVariant::SourceErrorVariant(variant) => {
                let name = &variant.name;
                let source = &variant.source;
                error_variant_tokens.append_all(quote::quote! {
                #name(#source),
                });
            }
            AstErrorEnumVariant::Variant(variant) => {
                error_variant_tokens.append_all(quote::quote! {
                #variant,
                })
            }
        }
    }
    token_stream.append_all(quote::quote! {
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
        if let AstErrorEnumVariant::SourceErrorVariant(variant) = variant {
            has_source_match_branches = true;
            let name = &variant.name;
            source_match_branches.append_all(quote::quote! {
                #enum_name::#name(ref source) => source.source(),
            });
        }
    }
    if has_source_match_branches {
        token_stream.append_all(quote::quote! {
            #[allow(unused_qualifications)]
            impl std::error::Error for #enum_name {
                fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                    match *self {
                        #source_match_branches
                        #[allow(unreachable_patterns)]
                        _ => None,
                    }
                }
            }
        });
    } else {
        token_stream.append_all(quote::quote! {
            #[allow(unused_qualifications)]
            impl std::error::Error for #enum_name {}
        });
    }
}

fn impl_display_and_debug(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
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
    for error_variant in error_variants {
        match error_variant {
            AstErrorEnumVariant::SourceErrorVariant(variant) => {
                let name = &variant.name;
                error_variant_tokens.append_all(quote::quote! {
                    #enum_name::#name(_) =>  concat!(stringify!(#enum_name), "::", stringify!(#name)),
                });
            }
            AstErrorEnumVariant::Variant(variant) => {
                error_variant_tokens.append_all(quote::quote! {
                    #enum_name::#variant =>  concat!(stringify!(#enum_name), "::", stringify!(#variant)),
                })
            }
        }
    }
    token_stream.append_all(quote::quote! {
        impl core::fmt::Display for #enum_name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                let variant_name = match *self {
                    #error_variant_tokens
                };
                write!(f, "{}", variant_name)
            }
        }

        impl core::fmt::Debug for #enum_name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                let variant_name = match *self {
                    #error_variant_tokens
                };
                write!(f, "{}", variant_name)
            }
        }
    });
}

fn impl_froms(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    let ErrorEnumGraphNode {
        error_enum,
        subsets,
    } = error_enum_node;

    let error_enum_name = &error_enum.error_name;
    for subset in (*subsets).iter() {
        let sub_error_enum = &(&*(**subset).borrow()).error_enum;
        let sub_error_variants = &sub_error_enum.error_variants;
        let sub_error_enum_name = &sub_error_enum.error_name;
        assert!(
            !sub_error_variants.is_empty(),
            "Error variants should not be empty"
        );
        let mut error_branch_tokens = TokenStream::new();
        for sub_error_variant in sub_error_variants {
            match sub_error_variant {
                // If sub error enum has a source variant, it must also exist in this error enum, but it may go by a different name.
                AstErrorEnumVariant::SourceErrorVariant(sub_error_variant) => {
                    let sub_error_variant_name = &sub_error_variant.name;
                    let error_variant_with_source_matching_sub_error_variant = error_enum.error_variants.iter().filter_map(|error_variant| {
                        match error_variant {
                            AstErrorEnumVariant::SourceErrorVariant(source_error_variant) => {
                                if is_type_path_equal(&source_error_variant.source, &sub_error_variant.source) {
                                    return Some(source_error_variant);
                                }
                                else {
                                    return None;
                                }
                            },
                            _ => None,
                        }}).next()
                    .expect("Logical error when creating the error enum graph. If one enum is a subset of another, any sources in the subset must exist in the super set.");
                    let error_variant_name = &error_variant_with_source_matching_sub_error_variant.name;
                    error_branch_tokens.append_all(quote::quote! {
                        #sub_error_enum_name::#sub_error_variant_name(source) =>  #error_enum_name::#error_variant_name(source),
                    });
                }
                AstErrorEnumVariant::Variant(sub_error_variant) => {
                    error_branch_tokens.append_all(quote::quote! {
                        #sub_error_enum_name::#sub_error_variant =>  #error_enum_name::#sub_error_variant,
                    })
                }
            }
        }
        token_stream.append_all(quote::quote! {
            impl From<#sub_error_enum_name> for #error_enum_name {
                fn from(error: #sub_error_enum_name) -> Self {
                    match error {
                        #error_branch_tokens
                    }
                }
            }
        });
    }
}
//************************************************************************//
#[derive(Clone)]
struct ErrorEnumGraphNode {
    pub(crate) error_enum: ErrorEnum,
    /// nodes where all error variants of the error enum are in this error enum's error variants.
    pub(crate) subsets: Vec<Rc<RefCell<ErrorEnumGraphNode>>>,
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
}

#[derive(Clone,Debug)]
pub(crate) struct ErrorEnum {
    pub(crate) error_name: Ident,
    pub(crate) error_variants: Vec<AstErrorEnumVariant>,
}

impl  std::hash::Hash for ErrorEnum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.error_name.hash(state);
    }
}

impl Eq for ErrorEnum {}

impl PartialEq for ErrorEnum {
    fn eq(&self, other: &Self) -> bool {
        self.error_name == other.error_name
    }
}