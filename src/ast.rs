use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Ident, Result,
};

pub struct ErrorSet {
    pub set_name: Ident,
    pub set_items: Punctuated<ErrorEnum, token::Comma>,
}

impl Parse for ErrorSet {
    fn parse(input: ParseStream) -> Result<Self> {
        let set_name: Ident = input.parse()?;
        let content;
        let _brace_token = braced!(content in input);
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
    pub error_name: Ident,
    pub error_variants: Punctuated<ErrorVariant, token::Comma>,
}

impl Parse for ErrorEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let error_name: Ident = input.parse()?;
        if input.peek(token::Comma) || input.is_empty() {
            let error_variants = Punctuated::<Ident,token::Comma>::new();
            return Ok(ErrorEnum {
                error_name,
                error_variants,
            });
        } else {
            let content;
            let _brace_token = braced!(content in input);
            let error_variants = content.parse_terminated(
                |input: ParseStream| input.parse::<ErrorVariant>(),
                token::Comma,
            )?;
            return Ok(ErrorEnum {
                error_name,
                error_variants,
            });
        }
    }
}
