mod ast;
mod expand;
mod resolve;
mod validate;

use ast::{AstErrorSet, AstErrorVariant, AstInlineErrorVariantField};
use expand::expand;

use quote::ToTokens;
use resolve::resolve;
use validate::validate;

#[proc_macro]
pub fn error_set(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Dev Note: If the macro is not updating when macro changes, uncomment below, rust-analyzer may be stuck and you need to restart: https://github.com/rust-lang/rust-analyzer/issues/10027
    // let token_stream: proc_macro2::TokenStream = syn::parse_str("const int: i32 = 1;").unwrap();
    // return proc_macro::TokenStream::from(token_stream);
    let error_set = syn::parse_macro_input!(tokens as AstErrorSet);
    let error_enums = match resolve(error_set) {
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

//************************************************************************//

pub(crate) fn is_source_tuple_type(error_variant: &AstErrorVariant) -> bool {
    return error_variant.source_type.is_some() && error_variant.fields.is_none();
}

/// To determine if [this] can be converted into [that] without dropping values.
/// Ignoring backtrace (since this is generated in the `From` impl if missing) and display.
/// This does not mean [this] is a subset of [that]. 
/// Why do they need to be exact?
/// e.g.
/// ```
/// X {
///   a: String,
///   b: u32,
/// }
/// ```
/// The above can be converted to the below, by droping the `b`. Even though the below could be considered a "subset".
/// ```
/// Y {
///   a: String
/// }
/// ```
/// If the below was also in the target enum, it would also be valid conversion target
/// ```
/// Z {
///  b: u32
/// }
/// ```
/// Thus, the names and shapes must be exactly the same to avoid this. 
/// Note, there can multiple source tuples or sources only struct of each wrapped error type.
/// The first that is encountered becomes the `From` impl of that source error type.
/// To ensure the correct one is selected, pay attention to `X = A || B` ordering 
/// or define your own `X = { IoError(std::io::Error) } || A || B`
/// 
/// Another example:
/// ```
///  N1 {
///     field: i32
///  }
/// ```
/// ==
/// ```
/// N1 {
///     field: i32
///  }
/// ```
/// !=
/// ```
/// N2 {
///     field: i32
///  }
/// ```
pub(crate) fn is_conversion_target(this: &AstErrorVariant, that: &AstErrorVariant) -> bool {
    return match (&this.source_type, &that.source_type) {
        (Some(this_source_type), Some(other_source_type)) => {
            this_source_type.path == other_source_type.path && this.name == that.name && this.fields == that.fields
        }
        (None, None) => {
            this.name == that.name && this.fields == that.fields
        }
        _ => false,
    };
}
