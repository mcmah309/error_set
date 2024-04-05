

use syn::{
    braced, parenthesized, parse::{discouraged::Speculative, Parse, ParseStream}, punctuated::Punctuated, token, Ident, Result
};

#[derive(Clone)]
pub(crate) struct AstErrorSet {
    pub(crate) set_items: Punctuated<AstErrorDeclaration, token::Colon>,
}

impl Parse for AstErrorSet {
    fn parse(input: ParseStream) -> Result<Self> {
        let set_items: Punctuated<AstErrorDeclaration, token::Colon> = input.parse_terminated(
            |input: ParseStream| input.parse::<AstErrorDeclaration>(),
            token::Colon,
        )?;
        Ok(AstErrorSet {
            set_items,
        })
    }
}

#[derive(Clone)]
pub(crate) struct AstErrorDeclaration {
    pub(crate) error_name: Ident,
    pub(crate) parts: Punctuated<AstInlineOrRefError, token::OrOr>,
}

impl Parse for AstErrorDeclaration {
    fn parse(input: ParseStream) -> Result<Self> {
        let error_name: Ident = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let content;
        let _brace_token = braced!(content in input);
        let parts = content.parse_terminated(
            |input: ParseStream| input.parse::<AstInlineOrRefError>(),
            token::OrOr,
        )?;
        return Ok(AstErrorDeclaration {
            error_name,
            parts,
        });
    }
}

pub(crate) type RefError = Ident;

#[derive(Clone)]
pub(crate) enum AstInlineOrRefError {
    Inline(AstInlineError),
    Ref(RefError),
}

impl Parse for AstInlineOrRefError {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        if let Ok(inline_error) = fork.parse::<AstInlineError>() {
            input.advance_to(&fork);
            return Ok(AstInlineOrRefError::Inline(inline_error));
        }
        match input.parse::<RefError>() {
            Ok(ref_error) => Ok(AstInlineOrRefError::Ref(ref_error)),
            Err(err) => Err(syn::parse::Error::new(
                err.span(),
                "Expected the error variants to be inline or a reference to another error enum.",
            )),
        }
    }
}


#[derive(Clone)]
pub(crate) struct AstInlineError {
    pub error_variants: Punctuated<AstErrorEnumVariant, token::Comma>
}

impl Parse for AstInlineError {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _brace_token = braced!(content in input);
        let error_variants = content.parse_terminated(
            |input: ParseStream| input.parse::<AstErrorEnumVariant>(),
            token::Comma,
        )?;
        return Ok(AstInlineError {
            error_variants,
        });
    }
}

pub(crate) type AstErrorVariant = Ident;

#[derive(Clone)]
pub(crate) enum AstErrorEnumVariant {
    SourceErrorVariant(AstSourceErrorVariant),
    Variant(AstErrorVariant),
}

impl Parse for AstErrorEnumVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        if let Ok(path) = fork.parse::<AstSourceErrorVariant>() {
            input.advance_to(&fork);
            return Ok(AstErrorEnumVariant::SourceErrorVariant(path));
        }
        match input.parse::<AstErrorVariant>() {
            Ok(error_variant) => Ok(AstErrorEnumVariant::Variant(error_variant)),
            Err(err) => Err(syn::parse::Error::new(
                err.span(),
                "Expected the error enum item to be a source error or error variant.",
            )),
        }
    }
}

impl PartialEq for AstErrorEnumVariant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                AstErrorEnumVariant::SourceErrorVariant(var1),
                AstErrorEnumVariant::SourceErrorVariant(var2),
            ) => {
                return is_type_path_equal(&var1.source, &var2.source);
            }
            (AstErrorEnumVariant::Variant(variant1), AstErrorEnumVariant::Variant(variant2)) => {
                variant1 == variant2
            }
            _ => false,
        }
    }
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

#[derive(Clone)]
pub(crate) struct AstSourceErrorVariant {
    pub(crate) name: Ident,
    pub(crate) source: syn::TypePath,
}

impl Parse for AstSourceErrorVariant{
    fn parse(input: ParseStream) -> Result<Self> {
        let name =  input.parse::<Ident>()?;
        let content;
        parenthesized!(content in input);
        let source = content.parse()?;
        //println!("path is {}",path.path.segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>().join("::"));
        Ok(AstSourceErrorVariant {
            name,
            source
        })
    }
}