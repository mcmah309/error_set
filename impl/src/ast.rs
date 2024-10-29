use proc_macro2::TokenStream;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token, Attribute, Ident, Result,
};

const DISPLAY_ATTRIBUTE_NAME: &str = "display";

#[derive(Clone)]
pub(crate) struct AstErrorSet {
    pub(crate) set_items: Vec<AstErrorDeclaration>,
}

impl Parse for AstErrorSet {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut set_items = Vec::new();
        while !input.is_empty() {
            let set_item = input.parse::<AstErrorDeclaration>()?;
            set_items.push(set_item);
            if input.parse::<token::Semi>().is_err() {
                return Err(syn::Error::new(
                    input.span(),
                    "Missing ending `;` for the set.",
                ));
            }
        }
        Ok(AstErrorSet { set_items })
    }
}

#[derive(Clone)]
pub(crate) struct AstErrorDeclaration {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) error_name: Ident,
    pub(crate) parts: Vec<AstInlineOrRefError>,
}

impl Parse for AstErrorDeclaration {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let save_position = input.fork();
        let error_name: Ident = input.parse()?;
        if !input.peek(syn::Token![=]) {
            return Err(syn::Error::new(
                save_position.span(),
                "Expected `=` to be next next.",
            ));
        }
        let last_position_save = input.fork();
        input.parse::<syn::Token![=]>().unwrap();
        let mut parts = Vec::new();
        while !input.is_empty() {
            let part = input.parse::<AstInlineOrRefError>()?;
            parts.push(part);
            if input.is_empty() {
                break;
            }
            if !input.peek(token::Semi) && !input.peek(token::OrOr) {
                return Err(syn::Error::new(
                    input.span(),
                    "Expected `;` or `||` to be next.",
                ));
            }
            if input.peek(token::Semi) {
                break;
            }
            input.parse::<token::OrOr>().unwrap();
        }
        if parts.is_empty() {
            return Err(syn::Error::new(
                last_position_save.span(),
                "Missing error definitions",
            ));
        }
        return Ok(AstErrorDeclaration {
            attributes,
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
        if input.peek(token::Brace) {
            return match input.parse::<AstInlineError>() {
                Ok(inline_error) => Ok(AstInlineOrRefError::Inline(inline_error)),
                Err(err) => Err(err),
            };
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
    pub error_variants: Punctuated<AstErrorVariant, token::Comma>,
}

impl Parse for AstInlineError {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let save_position = input.fork();
        let _brace_token = braced!(content in input);
        let error_variants = content.parse_terminated(
            |input: ParseStream| input.parse::<AstErrorVariant>(),
            token::Comma,
        )?;
        if error_variants.is_empty() {
            return Err(syn::parse::Error::new(
                save_position.span(),
                "Inline error variants cannot be empty",
            ));
        }
        return Ok(AstInlineError { error_variants });
    }
}

//************************************************************************//

/// A variant for an error
#[derive(Clone,Debug)] //todo remove debug
pub(crate) struct AstErrorVariant {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) display: Option<DisplayAttribute>,
    pub(crate) name: Ident,
    // Dev Note: `Some(Vec::new())` == `{}`, `Some(Vec::new(..))` == `{..}`, `None` == ``. `{}` means inline struct if has source as well.
    pub(crate) fields: Option<Vec<AstInlineErrorVariantField>>,
    pub(crate) source_type: Option<syn::TypePath>,
    #[allow(dead_code)] //todo remove
    pub(crate) backtrace_type: Option<syn::TypePath>,
}

impl Parse for AstErrorVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attributes = input.call(Attribute::parse_outer)?;
        let display = extract_display_attribute(&mut attributes)?;
        let name = input.parse::<Ident>()?;
        let content: syn::Result<_> = (|| {
            let content;
            parenthesized!(content in input);
            return Ok(content);
        })();
        let mut source_type = None;
        let mut backtrace_type = None;
        if let Ok(content) = content {
            let source_and_backtrace = content.parse_terminated(
                |input: ParseStream| input.parse::<syn::TypePath>(),
                token::Comma,
            );
            if let Ok(source_and_backtrace) = source_and_backtrace {
                if source_and_backtrace.len() <= 2 {
                    let mut source_and_backtrace = source_and_backtrace.into_iter();
                    source_type = source_and_backtrace.next();
                    backtrace_type = source_and_backtrace.next();
                } else {
                    return Err(syn::parse::Error::new(
                        source_and_backtrace.span(),
                        format!("Expected at most two elements - a source error type and a backtrace. Recieved {}.",source_and_backtrace.len() ),
                    ));
                }
            }
        }
        let content: syn::Result<_> = (|| {
            let content;
            syn::braced!(content in input);
            return Ok(content);
        })();
        let content = match content {
            Err(_) => {
                return Ok(AstErrorVariant {
                    attributes,
                    display,
                    name,
                    fields: None,
                    source_type,
                    backtrace_type,
                });
            }
            Ok(content) => content,
        };
        let fields = content
            .parse_terminated(AstInlineErrorVariantField::parse, syn::Token![,])?
            .into_iter()
            .collect::<Vec<_>>();
        let fields = Some(fields);
        Ok(AstErrorVariant {
            attributes,
            display,
            name,
            fields,
            source_type,
            backtrace_type,
        })
    }
}

//************************************************************************//

/// The format string to use for display
#[derive(Clone,Debug)]//todo remove debug
pub(crate) struct DisplayAttribute {
    pub(crate) tokens: TokenStream,
}

fn extract_display_attribute(
    attributes: &mut Vec<Attribute>,
) -> syn::Result<Option<DisplayAttribute>> {
    let mut display_indices = Vec::new();
    let mut displays = Vec::new();
    for (i, e) in attributes.iter().enumerate() {
        if let Some(display_tokens) = display_tokens(e) {
            displays.push(display_tokens);
            display_indices.push(i);
        }
    }
    if display_indices.is_empty() {
        return Ok(None);
    }
    let display = displays.remove(0);
    if display_indices.len() > 1 {
        return Err(syn::parse::Error::new(
            display.tokens.span(),
            format!("More than one `{}` attribute found", DISPLAY_ATTRIBUTE_NAME),
        ));
    }

    let mut index = 0;
    attributes.retain(|_| {
        let retain = !&display_indices.contains(&index);
        index += 1;
        return retain;
    });
    Ok(Some(display))
}

fn display_tokens(attribute: &Attribute) -> Option<DisplayAttribute> {
    return match &attribute.meta {
        syn::Meta::Path(_) => None,
        syn::Meta::NameValue(_) => None,
        syn::Meta::List(list) => {
            let ident = list.path.get_ident();
            let Some(ident) = ident else {
                return None;
            };
            let ident = ident.to_string();
            if &*ident == DISPLAY_ATTRIBUTE_NAME {
                return Some(DisplayAttribute {
                    tokens: list.tokens.clone(),
                });
            }
            return None;
        }
    };
}

#[derive(Clone, Debug, PartialEq)]//todo remove debug
pub(crate) struct AstInlineErrorVariantField {
    pub(crate) name: Ident,
    pub(crate) r#type: syn::Type,
}

impl Parse for AstInlineErrorVariantField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: syn::Token![:] = input.parse()?;
        let r#type: syn::Type = input.parse()?;
        Ok(AstInlineErrorVariantField { name, r#type })
    }
}

impl Eq for AstInlineErrorVariantField {}
