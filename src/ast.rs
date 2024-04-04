use std::error::Error;

use syn::{
    braced, parenthesized, parse::{discouraged::Speculative, Parse, ParseStream}, punctuated::Punctuated, token, Ident, Result
};

pub struct AstErrorSet {
    pub set_name: Ident,
    pub set_items: Punctuated<AstErrorSetItem, token::Comma>,
}

impl Parse for AstErrorSet {
    fn parse(input: ParseStream) -> Result<Self> {
        let set_name: Ident = input.parse()?;
        let content;
        let _brace_token = braced!(content in input);
        let set_items: Punctuated<AstErrorSetItem, token::Comma> = content.parse_terminated(
            |input: ParseStream| input.parse::<AstErrorSetItem>(),
            token::Comma,
        )?;
        Ok(AstErrorSet {
            set_name,
            set_items,
        })
    }
}

pub type AstErrorVariant = Ident;

pub enum AstErrorSetItem {
    SourceErrorVariant(AstSourceErrorVariant),
    ErrorEnum(AstErrorEnum),
    Variant(AstErrorVariant),
}

impl Parse for AstErrorSetItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut fork = input.fork();
        if let Ok(path) = fork.parse::<AstSourceErrorVariant>() {
            input.advance_to(&fork);
            return Ok(AstErrorSetItem::SourceErrorVariant(path));
        }
        fork = input.fork();
        if let Ok(error_enum) = fork.parse::<AstErrorEnum>() {
            input.advance_to(&fork);
            return Ok(AstErrorSetItem::ErrorEnum(error_enum));
        }
        match input.parse::<AstErrorVariant>() {
            Ok(error_variant) => Ok(AstErrorSetItem::Variant(error_variant)),
            Err(err) => Err(syn::parse::Error::new(
                err.span(),
                "Expected the error set item to be a error enum, source error, or error variant.",
            )),
        }
    }
}

#[derive(Clone)]
pub struct AstSourceErrorVariant {
    pub name: Ident,
    pub source: syn::TypePath,
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

#[derive(Clone)]
pub enum AstErrorEnumVariant {
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
                let segments1 = &var1.source.path.segments;
                let segments2 = &var2.source.path.segments;
                if segments1.len() != segments2.len() {
                    return false;
                }
                return segments1
                    .iter()
                    .zip(segments2.iter())
                    .all(|(seg1, seg2)| seg1.ident == seg2.ident);
            }
            (AstErrorEnumVariant::Variant(variant1), AstErrorEnumVariant::Variant(variant2)) => {
                variant1 == variant2
            }
            _ => false,
        }
    }
}

pub struct AstErrorEnum {
    pub error_name: Ident,
    pub error_variants: Punctuated<AstErrorEnumVariant, token::Comma>,
}

impl Parse for AstErrorEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let error_name: Ident = input.parse()?;
        let content;
        let _brace_token = braced!(content in input);
        let error_variants = content.parse_terminated(
            |input: ParseStream| input.parse::<AstErrorEnumVariant>(),
            token::Comma,
        )?;
        return Ok(AstErrorEnum {
            error_name,
            error_variants,
        });
    }
}

// #[derive(Debug,Clone)]
// struct X;

// impl Error for X {

// }