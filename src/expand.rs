use std::{
    cell::RefCell,
    rc::Rc,
};

use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::Ident;

use crate::ast::{ErrorSet, ErrorVariant};

pub fn expand(error_set: ErrorSet) -> TokenStream {
    let mut all_variants = Vec::new();
    for error_enum in error_set.set_items.iter() {
        for error_variant in error_enum.error_variants.iter() {
            if !all_variants.contains(error_variant) {
                all_variants.push(error_variant.clone());
            }
        }
    }
    let mut error_enum_nodes: Vec<Rc<RefCell<ErrorEnumGraphNode>>> = error_set
        .set_items
        .into_iter()
        .map(|e| Rc::new(RefCell::new(ErrorEnumGraphNode::new(e.into()))))
        .collect();
    //todo validate there are no duplicate error enums, do in ast
    // Add set level
    let set_level_node = ErrorEnumGraphNode::new(ErrorEnum {
        error_name: error_set.set_name,
        error_variants: all_variants,
    });
    error_enum_nodes.push(Rc::new(RefCell::new(set_level_node)));
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
                    .out_nodes
                    .push(checking_node.clone());
            }
        }
    }

    let mut token_stream = TokenStream::new();
    for error_enum_node in error_enum_nodes.iter() {
        add_code_for_node(&*(**error_enum_node).borrow(), &mut token_stream);
    }
    token_stream

    //syn::parse_str(&format!("struct Test({});",error_enum_nodes.len())).unwrap()
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
        out_nodes: _,
    } = error_enum_node;

    let enum_name = &error_enum.error_name;
    let error_variants = &error_enum.error_variants;
    token_stream.append_all(quote::quote! {
        #[derive(Debug,Clone,Eq,PartialEq,Hash)]
        pub enum #enum_name {
            #(
                #error_variants,
            )*
        }
    });
}

fn impl_error(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    let ErrorEnumGraphNode {
        error_enum,
        out_nodes: _,
    } = error_enum_node;

    let enum_name = &error_enum.error_name;
    token_stream.append_all(quote::quote! {
        #[allow(unused_qualifications)]
        impl std::error::Error for #enum_name {}
    });
}

fn impl_display(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    let ErrorEnumGraphNode {
        error_enum,
        out_nodes: _,
    } = error_enum_node;

    let enum_name = &error_enum.error_name;
    let variants = &error_enum.error_variants;
    token_stream.append_all(quote::quote! {
        impl core::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                let variant_name = match *self {
                    #(
                        #enum_name::#variants => "#enum_name::#variants",
                    )*
                };
                write!(f, "{}", variant_name)
            }
        }
    });
}

fn impl_froms(error_enum_node: &ErrorEnumGraphNode, token_stream: &mut TokenStream) {
    let ErrorEnumGraphNode {
        error_enum,
        out_nodes,
    } = error_enum_node;

    let enum_name = &error_enum.error_name;
    for out_node in (*out_nodes).iter() {
        let sub_error_enum = &(&*(**out_node).borrow()).error_enum;
        let error_variants = &sub_error_enum.error_variants;
        let subset_enum_name = &sub_error_enum.error_name;
        token_stream.append_all(quote::quote! {
            impl From<#subset_enum_name> for #enum_name {
                fn from(error: #subset_enum_name) -> Self {
                    match error {
                        #(
                            #subset_enum_name::#error_variants => #enum_name::#error_variants,
                        )*
                    }
                }
            }
        });
    }
}
#[derive(Debug, Clone, Eq)]
struct ErrorEnumGraphNode {
    pub error_enum: ErrorEnum,
    pub out_nodes: Vec<Rc<RefCell<ErrorEnumGraphNode>>>,
}

impl PartialEq for ErrorEnumGraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.error_enum == other.error_enum
    }
}

impl ErrorEnumGraphNode {
    pub fn new(node: ErrorEnum) -> ErrorEnumGraphNode {
        ErrorEnumGraphNode {
            error_enum: node,
            out_nodes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct ErrorEnum {
    pub error_name: Ident,
    pub error_variants: Vec<ErrorVariant>,
}

impl PartialEq for ErrorEnum {
    fn eq(&self, other: &Self) -> bool {
        self.error_name == other.error_name
    }
}

impl From<crate::ast::ErrorEnum> for ErrorEnum {
    fn from(value: crate::ast::ErrorEnum) -> Self {
        ErrorEnum {
            error_name: value.error_name,
            error_variants: value.error_variants.into_iter().collect(),
        }
    }
}
