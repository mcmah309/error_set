mod ast;
mod expand;
mod resolve;
mod validate;

use ast::AstErrorSet;
use expand::expand;
use quote::TokenStreamExt;
use resolve::resolve;
use validate::validate;

use crate::ast::AstErrorKind;

#[proc_macro]
pub fn error_set(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let error_set = syn::parse_macro_input!(tokens as AstErrorSet);
    let mut error_enum_decls = Vec::new();
    let mut error_struct_decls = Vec::new();
    for item in error_set.set_items.into_iter() {
        match item {
            AstErrorKind::Enum(error_enum_decl) => {
                error_enum_decls.push(error_enum_decl);
            }
            AstErrorKind::Struct(struct_decl) => {
                error_struct_decls.push(struct_decl);
            }
        }
    }
    let error_enums = match resolve(error_enum_decls) {
        Ok(ok) => ok,
        Err(err) => {
            return err.into_compile_error().into();
        }
    };
    if let Err(err) = validate(&error_enums) {
        return err.into_compile_error().into();
    }
    expand(error_enums, error_struct_decls).into()
}

#[proc_macro]
pub fn error_set_part(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let error_set = syn::parse_macro_input!(tokens as AstErrorSet);
    let mut token_stream = proc_macro2::TokenStream::new();
    for item in error_set.set_items.into_iter() {
        let name = match item {
            AstErrorKind::Enum(error_enum_decl) => {
                error_enum_decl.error_name
            }
            AstErrorKind::Struct(struct_decl) => {
                struct_decl.r#struct.ident
            }
        };
        token_stream.append_all(quote::quote! {
            use crate::error_set::#name;
        });
    }
    token_stream.into()
}