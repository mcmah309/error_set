extern crate proc_macro;

mod ast;
mod expand;
mod validate;

use std::{
    cell::{Cell, RefCell, RefMut},
    collections::{HashMap, HashSet},
    hash::Hasher,
    rc::Rc,
};

use ast::{AstErrorDeclaration, AstErrorEnumVariant, AstErrorSet, AstErrorVariant, RefError};
use expand::{expand, ErrorEnum};
use validate::validate;

#[proc_macro]
pub fn error_set(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let error_set = syn::parse_macro_input!(tokens as AstErrorSet);
    let error_enums = match construct_error_enums(error_set) {
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

fn construct_error_enums(error_set: AstErrorSet) -> syn::Result<Vec<ErrorEnum>> {
    let mut error_enums: Vec<ErrorEnumRc> = Vec::new();
    let mut needs_resolution: HashMap<ErrorEnumRc, Vec<RefError>> = HashMap::new(); // Use index instead of &mut ErrorEnum

    for declaration in error_set.set_items.into_iter() {
        let AstErrorDeclaration { error_name, parts } = declaration;
        let error_enum = ErrorEnum {
            error_name,
            error_variants: Vec::new(),
        };

        let cell_error_enum = ErrorEnumRc::new(error_enum);

        for part in parts.into_iter() {
            match part {
                ast::AstInlineOrRefError::Inline(inline_part) => {
                    let mut error_enum = cell_error_enum.get_mut();
                    error_enum
                        .error_variants
                        .extend(inline_part.error_variants.into_iter());
                }
                ast::AstInlineOrRefError::Ref(ref_part) => {
                    if let Some(ref_vec) = needs_resolution.get_mut(&cell_error_enum) {
                        ref_vec.push(ref_part);
                    } else {
                        needs_resolution.insert(cell_error_enum.clone(),  vec![ref_part]);
                    }
                }
            }
        }
        error_enums.push(cell_error_enum);
    }
    // for (index, ref_vec) in needs_resolution.into_iter() {
    //     for ref_entry in ref_vec.into_iter() {
    //         if let Some(ref_error_enum) = error_enums.iter().find(|e| e.error_name == ref_entry) {
    //             ref_error_enum.error_name
    //         } else {
    //             //todo error
    //         }
    //     }
    // }
    Ok(error_enums.into_iter().map(|e| unsafe { Rc::try_unwrap(e.rc).unwrap_unchecked().into_inner() }).collect())
}

// #[derive(PartialEq)]
struct ErrorEnumRc {
    pub rc: Rc<RefCell<ErrorEnum>>,
}

impl ErrorEnumRc {
    fn new(error_enum: ErrorEnum) -> Self {
        Self {
            rc: Rc::new(RefCell::new(error_enum)),
        }
    }

    fn get_mut(&self) -> RefMut<ErrorEnum> {
        return (*self.rc).borrow_mut();
    }
}

impl Clone for ErrorEnumRc {
    fn clone(&self) -> Self {
        Self {
            rc: self.rc.clone(),
        }
    }
}


impl PartialEq for ErrorEnumRc {
    fn eq(&self, other: &Self) -> bool {
        self.get_mut().error_name == other.get_mut().error_name
    }
}

impl std::hash::Hash for ErrorEnumRc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_mut().error_name.hash(state);
    }
}

impl Eq for ErrorEnumRc {}

// fn resolve(error_enum: &ErrorEnum, refs:Vec<RefError>, error_enums: HashMap<&ErrorEnum>) {

// }

// fn resolve_helper(error_enum: &ErrorEnum, refs:Vec<RefError>, visted: Vec<&ErrorEnum>) {

// }
