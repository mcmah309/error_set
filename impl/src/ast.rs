use proc_macro2::TokenStream;
use syn::{
    braced, parenthesized,
    parse::{discouraged::Speculative, Parse, ParseStream},
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
    pub error_variants: Punctuated<AstErrorEnumVariant, token::Comma>,
}

impl Parse for AstInlineError {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let save_position = input.fork();
        let _brace_token = braced!(content in input);
        let error_variants = content.parse_terminated(
            |input: ParseStream| input.parse::<AstErrorEnumVariant>(),
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

#[derive(Clone, Debug)]
pub(crate) enum AstErrorEnumVariant {
    WrappedVariant(AstWrappedErrorVariant),
    InlineVariant(AstInlineErrorVariant),
}

impl Parse for AstErrorEnumVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        if let Ok(variant) = fork.parse::<AstWrappedErrorVariant>() {
            input.advance_to(&fork);
            return Ok(AstErrorEnumVariant::WrappedVariant(variant));
        }
        match input.parse::<AstInlineErrorVariant>() {
            Ok(error_variant) => Ok(AstErrorEnumVariant::InlineVariant(error_variant)),
            Err(mut err) => {
                let high_level_error = syn::parse::Error::new(
                    err.span(),
                    "Expected the error enum variant to be an error variant wrapping another error or an inline error variant with optional fields.",
                );
                err.combine(high_level_error);
                Err(err)
            }
        }
    }
}

impl PartialEq for AstErrorEnumVariant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                AstErrorEnumVariant::WrappedVariant(var1),
                AstErrorEnumVariant::WrappedVariant(var2),
            ) => {
                // Does not include name, because we only care about the type, since each set can only have one of a type
                return is_type_path_equal(&var1.source, &var2.source);
            }
            (
                AstErrorEnumVariant::InlineVariant(variant1),
                AstErrorEnumVariant::InlineVariant(variant2),
            ) => variant1 == variant2,
            (AstErrorEnumVariant::WrappedVariant(_), AstErrorEnumVariant::InlineVariant(_)) => {
                false
            }
            (AstErrorEnumVariant::InlineVariant(_), AstErrorEnumVariant::WrappedVariant(_)) => {
                false
            }
        }
    }
}

impl Eq for AstErrorEnumVariant {}

impl std::hash::Hash for AstErrorEnumVariant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            AstErrorEnumVariant::WrappedVariant(source_error_variant) => {
                source_error_variant.name.hash(state);
            }
            AstErrorEnumVariant::InlineVariant(named_variant) => {
                named_variant.hash(state);
            }
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

/// Wrapper around another error type
#[derive(Clone)]
pub(crate) struct AstWrappedErrorVariant {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) name: Ident,
    pub(crate) source: syn::TypePath,
}

impl Parse for AstWrappedErrorVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let name = input.parse::<Ident>()?;
        let content;
        parenthesized!(content in input);
        let source = content.parse()?;
        //println!("path is {}",path.path.segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>().join("::"));
        Ok(AstWrappedErrorVariant {
            attributes,
            name,
            source,
        })
    }
}

impl std::fmt::Debug for AstWrappedErrorVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = &self
            .source
            .path
            .segments
            .iter()
            .map(|e| e.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        f.debug_struct("AstWrappedErrorVariant")
            .field("name", &self.name)
            .field("source", source)
            .finish()
    }
}

/// A regular named variant
#[derive(Clone)]
pub(crate) struct AstInlineErrorVariant {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) display: Option<DisplayAttribute>,
    pub(crate) name: Ident,
    pub(crate) fields: Vec<AstInlineErrorVariantField>,
}

impl Parse for AstInlineErrorVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attributes = input.call(Attribute::parse_outer)?;
        let display = extract_display_attribute(&mut attributes)?;
        let name = input.parse::<Ident>()?;
        let content: syn::Result<_> = (|| {
            let content;
            syn::braced!(content in input);
            return Ok(content);
        })();
        let content = match content {
            Err(_) => {
                return Ok(AstInlineErrorVariant {
                    attributes,
                    display,
                    name,
                    fields: Vec::new(),
                });
            }
            Ok(content) => content,
        };
        let fields = content
            .parse_terminated(AstInlineErrorVariantField::parse, syn::Token![,])?
            .into_iter()
            .collect::<Vec<_>>();
        Ok(AstInlineErrorVariant {
            attributes,
            display,
            name,
            fields,
        })
    }
}

impl std::hash::Hash for AstInlineErrorVariant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// todo add a note that an inline variant with fields can be compared but attributes and display are not used for equality
impl PartialEq for AstInlineErrorVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.fields == other.fields
    }
}

impl Eq for AstInlineErrorVariant {}

impl std::fmt::Debug for AstInlineErrorVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AstInlineErrorVariant")
            .field("name", &self.name)
            .finish()
    }
}

/// The format string to use for display
#[derive(Clone)]
pub(crate) struct DisplayAttribute {
    pub(crate) tokens: TokenStream,
}

// impl PartialEq for DisplayAttribute {
//     fn eq(&self, other: &Self) -> bool {
//         self.tokens.to_string() == other.tokens.to_string()
//     }
// }

// impl Eq for DisplayAttribute {}

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

#[derive(Clone)]
pub(crate) struct AstInlineErrorVariantField {
    pub(crate) name: Ident,
    pub(crate) r#type: syn::TypePath,
}

impl Parse for AstInlineErrorVariantField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: syn::Token![:] = input.parse()?;
        let r#type: syn::TypePath = input.parse()?;
        Ok(AstInlineErrorVariantField { name, r#type })
    }
}

impl PartialEq for AstInlineErrorVariantField {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self
                .r#type
                .path
                .segments
                .iter()
                .map(|e| &e.ident)
                .collect::<Vec<_>>()
                == other
                    .r#type
                    .path
                    .segments
                    .iter()
                    .map(|e| &e.ident)
                    .collect::<Vec<_>>()
    }
}

impl Eq for AstInlineErrorVariantField {}
