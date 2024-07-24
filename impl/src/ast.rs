use syn::{
    braced, parenthesized,
    parse::{discouraged::Speculative, Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Ident, Result,
};

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
                Err(err) => Err(err)
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
                // Does not include name, becuase we only care about the type, since each set can only have one of a type
                return is_type_path_equal(&var1.source, &var2.source);
            }
            (AstErrorEnumVariant::Variant(variant1), AstErrorEnumVariant::Variant(variant2)) => {
                variant1 == variant2
            }
            _ => false,
        }
    }
}

impl Eq for AstErrorEnumVariant {}

impl std::hash::Hash for AstErrorEnumVariant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            AstErrorEnumVariant::SourceErrorVariant(source_error_variant) => {
                source_error_variant.name.hash(state);
            }
            AstErrorEnumVariant::Variant(variant) => {
                variant.hash(state);
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

#[derive(Clone)]
pub(crate) struct AstErrorVariant {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) name: Ident,
}

impl Parse for AstErrorVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let name = input.parse::<Ident>()?;
        Ok(AstErrorVariant { attributes, name })
    }
}

impl std::hash::Hash for AstErrorVariant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for AstErrorVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for AstErrorVariant {}

impl std::fmt::Debug for AstErrorVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AstErrorVariant")
            .field("name", &self.name)
            .finish()
    }
}

#[derive(Clone)]
pub(crate) struct AstSourceErrorVariant {
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) name: Ident,
    pub(crate) source: syn::TypePath,
}

impl Parse for AstSourceErrorVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let name = input.parse::<Ident>()?;
        let content;
        parenthesized!(content in input);
        let source = content.parse()?;
        //println!("path is {}",path.path.segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>().join("::"));
        Ok(AstSourceErrorVariant {
            attributes,
            name,
            source,
        })
    }
}

impl std::fmt::Debug for AstSourceErrorVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = &self
            .source
            .path
            .segments
            .iter()
            .map(|e| e.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        f.debug_struct("AstSourceErrorVariant")
            .field("name", &self.name)
            .field("source", source)
            .finish()
    }
}