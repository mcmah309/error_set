use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, Item, Result, Token,
};

pub struct ErrorSet {
    set_name: Ident,
    set_items: Punctuated<ErrorEnum, token::Comma>,
}

impl Parse for ErrorSet {
    fn parse(input: ParseStream) -> Result<Self> {
        print!("hello");
        let set_name: Ident = input.parse()?;
        input.parse::<token::Comma>()?;
        let content;
        let brace_token = braced!(content in input);
        let set_items: Punctuated<ErrorEnum, token::Comma> = content.parse_terminated(
            |input: ParseStream| input.parse::<ErrorEnum>(),
            token::Comma,
        )?;
        Ok(ErrorSet {
            set_name,
            set_items,
        })
    }
}

pub type ErrorVariant = Ident;

pub struct ErrorEnum {
    error_name: Ident,
    error_variants: Punctuated<ErrorVariant, token::Comma>,
}

impl Parse for ErrorEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let error_name: Ident = input.parse()?;
        let content;
        let brace_token = braced!(content in input);
        let error_variants = content.parse_terminated(
            |input: ParseStream| input.parse::<ErrorVariant>(),
            token::Comma,
        )?;
        Ok(ErrorEnum {
            error_name,
            error_variants,
        })
    }
}
