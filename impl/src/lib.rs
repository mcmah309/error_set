mod ast;
mod expand;
mod resolve;
mod validate;

use ast::{AstErrorSet, AstErrorVariant};
use expand::expand;

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
    return error_variant.source_type.is_some() && error_variant.fields.is_empty();
}

// todo doc
/// Does not include backtrace since this is generated in the `From` impl if missing.
/// Does not include attributes, like display
pub(crate) fn is_this_a_subset_of_other(this: &AstErrorVariant, other: &AstErrorVariant) -> bool {
    match (&this.source_type, &other.source_type) {
        (Some(this_source_type), Some(other_source_type)) => {
            // Dev Note: Does not include anything else, because we only care about the type, since each set can only have one of a type.
            if is_type_path_equal(this_source_type, other_source_type) {
                return this.fields.iter().all(|e| other.fields.contains(e));
            }
            return false;
        }
        (None, None) => {}
        _ => return false,
    }
    return this.name == other.name && this.fields.iter().all(|e| other.fields.contains(e));
}

/// If the error defintions occupy the same space
/// 
/// - Only one error wrapped error variant of each source type is allowed. e.g. `V1(std::io::IoError)` V2(std::io::IoError)`
///   is a no go in the same error.
/// - Only one non-wrapping error variant of each name. e.g. `ParsingError` and `ParsingError` is not valid
pub(crate) fn does_occupy_the_same_space(
    this: &AstErrorVariant,
    other: &AstErrorVariant,
) -> bool {
    return match (&this.source_type, &other.source_type) {
        (Some(this_source_type), Some(other_source_type)) => {
            return is_type_path_equal(this_source_type, other_source_type) || this.name == other.name;
        }
        (None, None) => this.name == other.name,
        _ => return false,
    };
}

pub(crate) fn is_type_path_equal(path1: &syn::TypePath, path2: &syn::TypePath) -> bool {
    let segments1 = &path1.path.segments;
    let segments2 = &path2.path.segments;
    if segments1.len() != segments2.len() {
        return false;
    }
    return segments1
        .iter()
        .zip(segments2.iter())
        .all(|(seg1, seg2)| seg1.ident == seg2.ident);
}
