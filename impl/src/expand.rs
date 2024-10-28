use std::{cell::RefCell, rc::Rc};

#[cfg(feature = "coerce_macro")]
use coerce_macro::add_coerce_macro;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{Attribute, Ident, Lit};

use crate::ast::{is_type_path_equal, AstErrorEnumVariant};

/// Expand the [ErrorEnum]s into code.
pub(crate) fn expand(error_enums: Vec<ErrorEnum>) -> TokenStream {
    let mut token_stream = TokenStream::new();
    #[cfg(feature = "coerce_macro")]
    add_coerce_macro(&error_enums, &mut token_stream);
    let error_enum_nodes: Vec<Rc<RefCell<ErrorEnumGraphNode>>> = error_enums
        .into_iter()
        .map(|e| Rc::new(RefCell::new(ErrorEnumGraphNode::new(e.into()))))
        .collect();
    // build a graph of sub-sets and sets based on if all the variants of one are included in another
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
            AstErrorEnumVariant::WrappedVariant(variant) => {
                let name = &variant.name;
                let error_type = &variant.error_type;
                let attributes = &variant.attributes;
                error_variant_tokens.append_all(quote::quote! {
                    #(#attributes)*
                    #name(#error_type),
                });
            }
            AstErrorEnumVariant::InlineVariant(variant) => {
                let name = &variant.name;
                let attributes = &variant.attributes;
                if variant.fields.is_empty() {
                    error_variant_tokens.append_all(quote::quote! {
                        #(#attributes)*
                        #name,
                    });
                } else {
                    let field_names = &variant.fields.iter().map(|e| &e.name).collect::<Vec<_>>();
                    let field_types = &variant.fields.iter().map(|e| &e.r#type).collect::<Vec<_>>();
                    error_variant_tokens.append_all(quote::quote! {
                        #(#attributes)*
                        #name {
                            #(#field_names : #field_types),*
                        },
                    });
                }
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
        if let AstErrorEnumVariant::WrappedVariant(variant) = variant {
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
            impl core::error::Error for #enum_name {
                fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
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
            impl core::error::Error for #enum_name {}
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
            AstErrorEnumVariant::WrappedVariant(variant) => {
                let name = &variant.name;
                if let Some(display) = &variant.display {
                    let tokens = &display.tokens;
                    // e.g. `opaque`
                    if is_opaque(tokens.clone()) {
                        error_variant_tokens.append_all(quote::quote! {
                            #enum_name::#name(_) =>  write!(f, "{}", concat!(stringify!(#enum_name), "::", stringify!(#name))),
                        });
                    } else if let Some(string) = extract_string_if_str_literal(tokens.clone()) {
                        // e.g. `"{}"`
                        if is_format_str(&string) {
                            error_variant_tokens.append_all(quote::quote! {
                                #enum_name::#name(ref source) =>  write!(f, #tokens, source),
                            });
                        } else {
                            // e.g. `"literal str"`
                            error_variant_tokens.append_all(quote::quote! {
                                #enum_name::#name(_) =>  write!(f, "{}", #tokens),
                            });
                        }
                    } else {
                        // e.g. `"field: {}", err.field`
                        error_variant_tokens.append_all(quote::quote! {
                            #enum_name::#name(ref source) =>  write!(f, #tokens),
                        });
                    }
                } else {
                    error_variant_tokens.append_all(quote::quote! {
                        #enum_name::#name(ref source) =>  write!(f, "{}", source),
                    });
                }
            }
            AstErrorEnumVariant::InlineVariant(variant) => {
                let name = &variant.name;
                if let Some(display) = &variant.display {
                    if variant.fields.is_empty() {
                        let tokens = &display.tokens;
                        // `"literal str"`
                        if is_str_literal(tokens.clone()) {
                            error_variant_tokens.append_all(quote::quote! {
                            #enum_name::#name =>  write!(f, "{}", #tokens),
                            });
                        // `"My name is {}", func()`
                        } else {
                            error_variant_tokens.append_all(quote::quote! {
                            #enum_name::#name => write!(f, #tokens),
                            });
                        }
                    } else {
                        let field_names =
                            &variant.fields.iter().map(|e| &e.name).collect::<Vec<_>>();
                        let tokens = &display.tokens;
                        if let Some(string) = extract_string_if_str_literal(tokens.clone()) {
                            // `"My name is {name}"`
                            if is_format_str(&string) {
                                error_variant_tokens.append_all(quote::quote! {
                                #enum_name::#name { #(ref #field_names),*  } =>  write!(f, #tokens),
                                });
                            // `"literal str"`
                            } else {
                                error_variant_tokens.append_all(quote::quote! {
                                #enum_name::#name { #(ref #field_names),*  } =>  write!(f, "{}", #tokens),
                                });
                            }
                        // `"My name is {}", name`
                        } else {
                            error_variant_tokens.append_all(quote::quote! {
                            #enum_name::#name { #(ref #field_names),*  } =>  write!(f, #tokens),
                            });
                        }
                    }
                } else {
                    if variant.fields.is_empty() {
                        error_variant_tokens.append_all(quote::quote! {
                            #enum_name::#name =>  write!(f, "{}", concat!(stringify!(#enum_name), "::", stringify!(#name))),
                            });
                    } else {
                        let field_names =
                            &variant.fields.iter().map(|e| &e.name).collect::<Vec<_>>();
                        error_variant_tokens.append_all(quote::quote! {
                        #enum_name::#name { #(ref #field_names),*  } =>  write!(f, "{}", concat!(stringify!(#enum_name), "::", stringify!(#name))),
                        });
                    }
                }
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

fn impl_froms(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    let ErrorEnumGraphNode {
        error_enum,
        subsets,
    } = error_enum_node;

    let error_enum_name = &error_enum.error_name;
    // Add all `From`'s for the enums that are a subset of this one
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
                // If sub error enum has a source variant, it must also exist in this error enum (otherwise it would not be a sub), but it may go by a different name.
                AstErrorEnumVariant::WrappedVariant(sub_error_variant) => {
                    let sub_error_variant_name = &sub_error_variant.name;
                    let error_variant_with_source_matching_sub_error_variant = error_enum.error_variants.iter().filter_map(|error_variant| {
                        match error_variant {
                            AstErrorEnumVariant::WrappedVariant(source_error_variant) => {
                                if is_type_path_equal(&source_error_variant.error_type, &sub_error_variant.error_type) {
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
                AstErrorEnumVariant::InlineVariant(sub_error_variant) => {
                    let sub_error_variant_name = &sub_error_variant.name;
                    if sub_error_variant.fields.is_empty() {
                        error_branch_tokens.append_all(quote::quote! {
                        #sub_error_enum_name::#sub_error_variant_name =>  #error_enum_name::#sub_error_variant_name,
                    });
                    } else {
                        let field_names = &sub_error_variant
                            .fields
                            .iter()
                            .map(|e| &e.name)
                            .collect::<Vec<_>>();
                        error_branch_tokens.append_all(quote::quote! {
                        #sub_error_enum_name::#sub_error_variant_name { #(#field_names),*  } =>  #error_enum_name::#sub_error_variant_name { #(#field_names),*  },
                        });
                    }
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
    // Add all `From`'s for variants that are wrappers around source errors.
    for source in error_enum.error_variants.iter().filter_map(|e| {
        return match e {
            AstErrorEnumVariant::WrappedVariant(source_variant) => {
                return Some(source_variant);
            }
            _ => None,
        };
    }) {
        let variant_name = &source.name;
        let error_type = &source.error_type;
        token_stream.append_all(quote::quote! {
            impl From<#error_type> for #error_enum_name {
                fn from(error: #error_type) -> Self {
                    #error_enum_name::#variant_name(error)
                }
            }
        })
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

// Dev Note: naive implementaion.
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
