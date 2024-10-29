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

/// To determine if [this] can be converted into [that].
/// This does not mean [this] is a subset of [that]. e.g.
/// ```
/// X {
///   a: String,
///   b: u32,
/// }
/// ```
/// The above can be converted to the below, by droping the `b`. Even though the below could be considered a "subset"
/// ```
/// Y {
///   a: String
/// }
/// ```
///
/// Note:
/// - Does not include backtrace since this is generated in the `From` impl if missing.
/// - Does not include attributes, like display.
/// - Names matter on inline structs:
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
pub(crate) fn can_convert_this_into_that(this: &AstErrorVariant, that: &AstErrorVariant) -> bool {
    // fn all_of_this_is_in_that(
    //     this: &Option<Vec<AstInlineErrorVariantField>>,
    //     that: &Option<Vec<AstInlineErrorVariantField>>,
    // ) -> bool {
    //     let x = Vec::new();
    //     let this_fields = this.as_ref().unwrap_or(&x);
    //     let that_fields = that.as_ref().unwrap_or(&x);
    //     //return that_fields.iter().all(|e| this_fields.contains(e));
    //     //panic!("this: {this_fields:?}\n\nthat:{that_fields:?}");
    //     return this_fields.iter().all(|e| that_fields.contains(e));
    // }
    //todo match if single tupe inner type the same or the shape is the same.
    return match (&this.source_type, &that.source_type) {
        (Some(this_source_type), Some(other_source_type)) => {
            // Dev Note: Does not include anything else, because we only care about the type, since each set can only have one of a type.
            this_source_type.path == other_source_type.path
                // && ((this.fields.is_none() == that.fields.is_none()
                //     || this.fields.is_some_and(|e| e.is_empty())
                //         && that.fields.is_some_and(|e| e.is_empty())) || (this.fields.is_some_and(|e|)))
            // && all_of_this_is_in_that(&this.fields, &that.fields)
        }
        (None, None) => {
            this.name == that.name
            // && all_of_this_is_in_that(&this.fields, &that.fields)
        }
        _ => false,
    };
}
