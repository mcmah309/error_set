use proc_macro2::TokenStream;

use crate::ast::ErrorSet;



pub fn expand(error_set: ErrorSet) -> TokenStream {
    syn::parse_str("struct Test(i32);").unwrap() //todo add fallback
}