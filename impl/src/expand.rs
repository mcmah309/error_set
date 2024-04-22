use std::{cell::RefCell, rc::Rc};

use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{Attribute, Ident};

use crate::ast::{is_type_path_equal, AstErrorEnumVariant};

pub(crate) fn expand(error_enums: Vec<ErrorEnum>) -> TokenStream {
    let enum_intersections = construct_set_intersections(&error_enums);
    let mut token_stream = TokenStream::new();
    add_coerce_macro(enum_intersections, &mut token_stream);
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

    for error_enum_node in error_enum_nodes.iter() {
        add_code_for_node(&*(**error_enum_node).borrow(), &mut token_stream);
    }
    token_stream
}

fn add_code_for_node(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    add_enum(error_enum_node, token_stream);
    impl_error(error_enum_node, token_stream);
    impl_display(error_enum_node, token_stream);
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
                let attributes = &variant.attributes;
                error_variant_tokens.append_all(quote::quote! {
                    #(#attributes)*
                    #name(#source),
                });
            }
            AstErrorEnumVariant::Variant(variant) => {
                let name = &variant.name;
                let attributes = &variant.attributes;
                error_variant_tokens.append_all(quote::quote! {
                    #(#attributes)*
                    #name,
                })
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

        impl error_set::ErrorSetMarker for #enum_name {}
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
    for error_variant in error_variants {
        match error_variant {
            AstErrorEnumVariant::SourceErrorVariant(variant) => {
                let name = &variant.name;
                error_variant_tokens.append_all(quote::quote! {
                    #enum_name::#name(_) =>  concat!(stringify!(#enum_name), "::", stringify!(#name)),
                });
            }
            AstErrorEnumVariant::Variant(variant) => {
                let name = &variant.name;
                error_variant_tokens.append_all(quote::quote! {
                    #enum_name::#name =>  concat!(stringify!(#enum_name), "::", stringify!(#name)),
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
                    let error_variant_name =
                        &error_variant_with_source_matching_sub_error_variant.name;
                    error_branch_tokens.append_all(quote::quote! {
                        #sub_error_enum_name::#sub_error_variant_name(source) =>  #error_enum_name::#error_variant_name(source),
                    });
                }
                AstErrorEnumVariant::Variant(sub_error_variant) => {
                    let sub_error_variant_name = &sub_error_variant.name;
                    error_branch_tokens.append_all(quote::quote! {
                        #sub_error_enum_name::#sub_error_variant_name =>  #error_enum_name::#sub_error_variant_name,
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
    for source in error_enum.error_variants.iter().filter_map(|e| {
        return match e {
            AstErrorEnumVariant::SourceErrorVariant(source_variant) => {
                return Some(source_variant);
            }
            _ => None,
        };
    }) {
        let variant_name = &source.name;
        let source = &source.source;
        token_stream.append_all(quote::quote! {
            impl From<#source> for #error_enum_name {
                fn from(error: #source) -> Self {
                    #error_enum_name::#variant_name(error)
                }
            }
        })
    }
}

//************************************************************************//

fn add_coerce_macro(enum_intersections: Vec<EnumIntersection>, token_stream: &mut TokenStream) {
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
                AstErrorEnumVariant::SourceErrorVariant(source_variant) => {
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
                },
                AstErrorEnumVariant::Variant(variant) => {
                    let variant = variant.name;
                    match_arms_return_err.append_all(quote::quote! {
                        Err(#enum1_name::#variant) => { return Err(#enum2_name::#variant); },
                    });
                    match_arms_err.append_all(quote::quote! {
                        Err(#enum1_name::#variant) => { Err(#enum2_name::#variant) },
                    });
                    match_arms_return.append_all(quote::quote! {
                        #enum1_name::#variant => { return #enum2_name::#variant; },
                    });
                    match_arms.append_all(quote::quote! {
                        #enum1_name::#variant => { #enum2_name::#variant },
                    }); 
                },
            }
        }
        macro_pattern_token_stream.append_all(quote::quote! {
                ($expr:expr => { $($patterns:pat => $results:expr$(,)?)+, {Err(#enum1_name) => return Err(#enum2_name)} }) => {
                    match $expr {
                        $($patterns => $results,)+
                        #match_arms_return_err
                    }
                };
                ($expr:expr => { $($patterns:pat => $results:expr$(,)?)+, {Err(#enum1_name) => Err(#enum2_name)} }) => {
                    match $expr {
                        $($patterns => $results,)+
                        #match_arms_err
                    }
                };
                ($expr:expr => { $($patterns:pat => $results:expr$(,)?)+, {#enum1_name => return #enum2_name} }) => {
                    match $expr {
                        $($patterns => $results,)+
                        #match_arms_return
                    }
                };
                ($expr:expr => { $($patterns:pat => $results:expr$(,)?)+, {#enum1_name => #enum2_name} }) => {
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
                if enum2.error_variants.contains(&variant)  {
                    intersections.push(variant.clone());
                }
            }
            if !intersections.is_empty() {
            let enum_intersection = EnumIntersection::new(enum1.error_name.clone(), enum2.error_name.clone(), intersections);
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
    pub(crate) fn new(enum1: Ident, enum2: Ident, intersection: Vec<AstErrorEnumVariant>) -> EnumIntersection {
        EnumIntersection {
            enum1,
            enum2,
            intersection,
        }
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

#[derive(Clone)]
pub(crate) struct ErrorEnum {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) error_name: Ident,
    pub(crate) error_variants: Vec<AstErrorEnumVariant>,
}

impl std::hash::Hash for ErrorEnum {
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
