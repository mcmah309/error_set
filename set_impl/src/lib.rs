mod enum_error_set;

#[cfg(feature = "error_set")]
#[proc_macro]
pub fn error_set(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Dev Note: If the macro is not updating when macro changes, uncomment below, rust-analyzer may be stuck and you need to restart: https://github.com/rust-lang/rust-analyzer/issues/10027
    // let token_stream: proc_macro2::TokenStream = syn::parse_str("const int: i32 = 1;").unwrap();
    // return proc_macro::TokenStream::from(token_stream);
    let error_set = syn::parse_macro_input!(tokens as enum_error_set::AstErrorSet);
    let error_enums = match enum_error_set::resolve(error_set) {
        Ok(ok) => ok,
        Err(err) => {
            return err.into_compile_error().into();
        }
    };
    if let Err(err) = enum_error_set::validate(&error_enums) {
        return err.into_compile_error().into();
    }
    enum_error_set::expand(error_enums).into()
}
