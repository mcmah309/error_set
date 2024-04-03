use std::{
    borrow::Borrow,
    cell::{OnceCell, Ref, RefCell},
    collections::HashSet,
    ops::Deref,
    rc::Rc,
};

use proc_macro2::TokenStream;
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
    //todo validate there are no duplicate error enums
    // Add set level
    let set_level_node = ErrorEnumGraphNode::new(ErrorEnum {
        error_name: error_set.set_name,
        error_variants: all_variants
    });
    error_enum_nodes.push(Rc::new(RefCell::new(set_level_node)));
    for building_node in error_enum_nodes.iter() {
        for checking_node in error_enum_nodes.iter() {
            if (*(**checking_node).borrow()).value
                != (*(**building_node).borrow()).value
                && (*(**checking_node).borrow())
                    .value
                    .error_variants
                    .iter()
                    .all(|e| {
                        (*(**building_node).borrow())
                            .value
                            .error_variants
                            .contains(e)
                    })
            {
                building_node
                    .borrow_mut()
                    .out_edges
                    .push(checking_node.clone());
            }
        }
    }

    let mut combined = quote::quote! {}; // Create an empty TokenStream.
    for error_enum_node in error_enum_nodes.iter() {
        combined.extend(add_code_for_node(&*(**error_enum_node).borrow()));
    }
    combined

    //syn::parse_str(&format!("struct Test({});",error_enum_nodes.len())).unwrap()
}

fn add_code_for_node(error_enum_node: &ErrorEnumGraphNode) -> TokenStream {
    let ErrorEnumGraphNode {
        value: error_enum,
        out_edges: edges,
    } = error_enum_node;

    let mut froms: Vec<TokenStream> = Vec::new();
    for edge in (*edges).iter() {
        let edge = &*(**edge).borrow();
        let subset_enum = &edge.value;
        let error_variants = &subset_enum.error_variants;
        let subset_enum_name = &subset_enum.error_name;
        let enum_name = &error_enum.error_name;
        let stream = quote::quote! {
            impl From<#subset_enum_name> for #enum_name {
                fn from(error: #subset_enum_name) -> Self {
                    match error {
                        #(
                            #subset_enum_name::#error_variants => #enum_name::#error_variants,
                        )*
                    }
                }
            }
        };
        froms.push(stream)
    }
    let combined_froms: TokenStream = quote::quote! {
        #(#froms)*
    };
    combined_froms
}

#[derive(Debug, Clone, Eq)]
struct ErrorEnumGraphNode {
    pub value: ErrorEnum,
    pub out_edges: Vec<Rc<RefCell<ErrorEnumGraphNode>>>,
}

impl PartialEq for ErrorEnumGraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl ErrorEnumGraphNode {
    pub fn new(node: ErrorEnum) -> ErrorEnumGraphNode {
        ErrorEnumGraphNode {
            value: node,
            out_edges: Vec::new(),
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
