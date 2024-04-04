extern crate proc_macro;

mod ast;
mod expand;
mod validate;

use ast::AstErrorSet;
use expand::expand;
use validate::validate;

#[proc_macro]
pub fn error_set(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let error_set = syn::parse_macro_input!(tokens as AstErrorSet);
    if let Err(err) = validate(&error_set) {
        return err.into_compile_error().into();
    }
    expand(error_set).into()
}