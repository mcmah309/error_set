extern crate proc_macro;

mod ast;
mod expand;

use ast::ErrorSet;
use expand::expand;

#[proc_macro]
pub fn error_set(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let error_set = syn::parse_macro_input!(tokens as ErrorSet);
    expand(error_set).into()
}