mod ast;
mod expand;
mod resolve;
mod validate;

use ast::AstErrorSet;
use expand::expand;
use resolve::resolve;
use validate::validate;

use crate::ast::AstErrorKind;

#[proc_macro]
pub fn error_set(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Dev Note: If the macro is not updating when macro changes, uncomment below, rust-analyzer may be stuck and you need to restart: https://github.com/rust-lang/rust-analyzer/issues/10027
    // let token_stream: proc_macro2::TokenStream = syn::parse_str("const int: i32 = 1;").unwrap();
    // return proc_macro::TokenStream::from(token_stream);
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
    expand(error_enums).into()
}
