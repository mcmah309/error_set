extern crate proc_macro;

mod ast;
mod expand;
mod validate;

use ast::AstErrorSet;
use expand::{expand, ErrorEnum};
use validate::validate;

#[proc_macro]
pub fn error_set(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let error_set = syn::parse_macro_input!(tokens as AstErrorSet);
    let error_enums = construct_error_enums(error_set);
    if let Err(err) = validate(&error_enums) {
        return err.into_compile_error().into();
    }
    expand(error_enums).into()
}

pub(crate) fn construct_error_enums(error_set: AstErrorSet) -> Vec<ErrorEnum> {
    error_set.set_items.into_iter().map(Into::into).collect::<Vec<_>>()
}

impl From<crate::ast::AstErrorEnum> for ErrorEnum {
    fn from(value: crate::ast::AstErrorEnum) -> Self {
        ErrorEnum {
            error_name: value.error_name,
            error_variants: value.error_variants.into_iter().collect(),
        }
    }
}