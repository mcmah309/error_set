# Error Set

[<img alt="github" src="https://img.shields.io/badge/github-mcmah309/error_set-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mcmah309/error_set)
[<img alt="crates.io" src="https://img.shields.io/crates/v/error_set.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/error_set)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-error_set-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/error_set)
[<img alt="test status" src="https://img.shields.io/github/actions/workflow/status/mcmah309/error_set/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/mcmah309/error_set/actions?query=branch%3Amaster)

Error Set simplifies error management by providing a streamlined method for defining errors and easily converting between them.

Error Set is inspired by Zig's [error set](https://ziglang.org/documentation/master/#Error-Set-Type), and offers similar functionality.

Instead of defining various enums/structs for errors and hand rolling relations, use an error set:
```rust
use error_set::error_set;

error_set! {
    /// The syntax below aggregates the referenced error variants
    MediaError := DownloadError || BookParsingError

    /// Since all variants in [DownloadError] are in [MediaError], a
    /// [DownloadError] can be turned into a [MediaError] with just `.into()` or `?`. 
    DownloadError := {
        #[display("Easily add custom display messages")]
        InvalidUrl,
        /// The `From` trait for `std::io::Error` will also be automatically derived
        #[display("Display messages work just like the `format!` macro {0}")]
        IoError(std::io::Error),
    }

    /// Traits like `Debug`, `Display`, `Error`, and `From` are all automatically derived
    #[derive(Clone)]
    BookParsingError := { MissingBookDescription, } || BookSectionParsingError

    BookSectionParsingError := {
        /// Inline structs are also supported
        #[display("Display messages can also reference fields, like {field}")]
        MissingField {
            field: String
        },
        NoContent,
    }
}
```
<details>

  <summary>Cargo Expand</summary>

```rust
#[doc = " The syntax below aggregates the referenced error variants"]
#[derive(Debug)]
pub enum MediaError {
    InvalidUrl, #[doc = " The `From` trait for `std::io::Error` will also be automatically derived"]
    IoError(std::io::Error),MissingBookDescription, #[doc = " Inline structs are also supported"]
    MissingField {
        field:String
    },NoContent,
}
#[allow(unused_qualifications)]
impl core::error::Error for MediaError {
    fn source(&self) -> Option< &(dyn core::error::Error+'static)>{
        match self {
            MediaError::IoError(source) => source.source(), 
            #[allow(unreachable_patterns)]
            _ => None,
        
            }
    }

    }
impl core::fmt::Display for MediaError {
    #[inline]
    fn fmt(&self,f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match& *self {
            MediaError::InvalidUrl => write!(f,"{}","Easily add custom display messages"),
            MediaError::IoError(source) => write!(f,"Display messages work just like the `format!` macro {0}",source),
            MediaError::MissingBookDescription => write!(f,"{}",concat!(stringify!(MediaError),"::",stringify!(MissingBookDescription))),
            MediaError::MissingField {
                field
            } => write!(f,"Display messages can also reference fields, like {field}"),
            MediaError::NoContent => write!(f,"{}",concat!(stringify!(MediaError),"::",stringify!(NoContent))),
        
            }
    }

    }
impl From<DownloadError>for MediaError {
    fn from(error:DownloadError) -> Self {
        match error {
            DownloadError::InvalidUrl => MediaError::InvalidUrl,
            DownloadError::IoError(source) => MediaError::IoError(source),
        
            }
    }

    }
impl From<BookParsingError>for MediaError {
    fn from(error:BookParsingError) -> Self {
        match error {
            BookParsingError::MissingBookDescription => MediaError::MissingBookDescription,
            BookParsingError::MissingField {
                field
            } => MediaError::MissingField {
                field
            },
            BookParsingError::NoContent => MediaError::NoContent,
        
            }
    }

    }
impl From<BookSectionParsingError>for MediaError {
    fn from(error:BookSectionParsingError) -> Self {
        match error {
            BookSectionParsingError::MissingField {
                field
            } => MediaError::MissingField {
                field
            },
            BookSectionParsingError::NoContent => MediaError::NoContent,
        
            }
    }

    }
impl From<std::io::Error>for MediaError {
    fn from(error:std::io::Error) -> Self {
        MediaError::IoError(error)
    }

    }
#[doc = " Since all variants in [DownloadError] are in [MediaError], a"]
#[doc = " [DownloadError] can be turned into a [MediaError] with just `.into()` or `?`."]
#[derive(Debug)]
pub enum DownloadError {
    InvalidUrl, #[doc = " The `From` trait for `std::io::Error` will also be automatically derived"]
    IoError(std::io::Error),
}
#[allow(unused_qualifications)]
impl core::error::Error for DownloadError {
    fn source(&self) -> Option< &(dyn core::error::Error+'static)>{
        match self {
            DownloadError::IoError(source) => source.source(), 
            #[allow(unreachable_patterns)]
            _ => None,
        
            }
    }

    }
impl core::fmt::Display for DownloadError {
    #[inline]
    fn fmt(&self,f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match& *self {
            DownloadError::InvalidUrl => write!(f,"{}","Easily add custom display messages"),
            DownloadError::IoError(source) => write!(f,"Display messages work just like the `format!` macro {0}",source),
        
            }
    }

    }
impl From<std::io::Error>for DownloadError {
    fn from(error:std::io::Error) -> Self {
        DownloadError::IoError(error)
    }

    }
#[doc = " Traits like `Debug`, `Display`, `Error`, and `From` are all automatically derived"]
#[derive(Clone)]
#[derive(Debug)]
pub enum BookParsingError {
    MissingBookDescription, #[doc = " Inline structs are also supported"]
    MissingField {
        field:String
    },NoContent,
}
#[allow(unused_qualifications)]
impl core::error::Error for BookParsingError{}

impl core::fmt::Display for BookParsingError {
    #[inline]
    fn fmt(&self,f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match& *self {
            BookParsingError::MissingBookDescription => write!(f,"{}",concat!(stringify!(BookParsingError),"::",stringify!(MissingBookDescription))),
            BookParsingError::MissingField {
                field
            } => write!(f,"Display messages can also reference fields, like {field}"),
            BookParsingError::NoContent => write!(f,"{}",concat!(stringify!(BookParsingError),"::",stringify!(NoContent))),
        
            }
    }

    }
impl From<BookSectionParsingError>for BookParsingError {
    fn from(error:BookSectionParsingError) -> Self {
        match error {
            BookSectionParsingError::MissingField {
                field
            } => BookParsingError::MissingField {
                field
            },
            BookSectionParsingError::NoContent => BookParsingError::NoContent,
        
            }
    }

    }
#[derive(Debug)]
pub enum BookSectionParsingError {
    #[doc = " Inline structs are also supported"]
    MissingField {
        field:String
    },NoContent,
}
#[allow(unused_qualifications)]
impl core::error::Error for BookSectionParsingError{}

impl core::fmt::Display for BookSectionParsingError {
    #[inline]
    fn fmt(&self,f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match& *self {
            BookSectionParsingError::MissingField {
                field
            } => write!(f,"Display messages can also reference fields, like {field}"),
            BookSectionParsingError::NoContent => write!(f,"{}",concat!(stringify!(BookSectionParsingError),"::",stringify!(NoContent))),
        
            }
    }

    }
```
</details>

The above error set can also be written as the full expansion (without the `||` operator).

<details>

  <summary>Full Expansion Representation</summary>

\*Comments and messages removed for brevity\*

```rust
error_set::error_set! {
    MediaError := {
        InvalidUrl,
        IoError(std::io::Error),
        MissingBookDescription,
        MissingField {
            field: String
        },
        NoContent,
    }
    DownloadError := {
        InvalidUrl,
        IoError(std::io::Error),
    }
    BookParsingError := {
        MissingBookDescription,
        MissingField {
            field: String
        },
        NoContent,
    }
    BookSectionParsingError := {
        MissingField {
            field: String
        },
        NoContent,
    }
}
```
</details>

Any above subset can be converted into a superset with `.into()` or `?`. 
This makes correctly scoping and passing up call chains a breeze.

<details>

  <summary>Basic Example</summary>

```rust
use error_set::{error_set, CoerceResult};

error_set! {
    MediaError := DownloadError || BookParsingError
    DownloadError := {
        InvalidUrl,
        IoError(std::io::Error),
    }
    BookParsingError := { MissingBookDescription, } || BookSectionParsingError
    BookSectionParsingError := {
        MissingField {
            field: String
        },
        NoContent,
    }
}

fn main() {
    let book_section_parsing_error: BookSectionParsingError =
        BookSectionParsingError::MissingField {
            field: "author".to_string(),
        };
    let book_parsing_error: BookParsingError = book_section_parsing_error.into();
    assert!(matches!(
        book_parsing_error,
        BookParsingError::MissingField { field: _ }
    ));
    let media_error: MediaError = book_parsing_error.into();
    assert!(matches!(media_error, MediaError::MissingField { field: _ }));

    let io_error = std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory");
    let result_download_error: Result<(), DownloadError> = Err(io_error).coerce(); // == .map_err(Into::into);
    let result_media_error: Result<(), MediaError> = result_download_error.map_err(Into::into);
    assert!(matches!(result_media_error, Err(MediaError::IoError(_))));
}
```
</details>


The typical project approach is to have one `errors.rs` file with a single `error_set`. This keeps
all the errors in one place and allows your IDE to autocomplete `crate::errors::` with of all errors.
But `error_set!` can also be used for quick errors "unions", no longer requiring users to 
hand write `From<..>` or use `.map_err(..)` for these simple cases.
e.g.
```rust
use std::collections::HashMap;
use jsonwebtoken::DecodingKey;

error_set::error_set! {
    JwtVerifierCreationError := {
        Reqwest(reqwest::Error),
        Jwt(jsonwebtoken::errors::Error),
    }
}

impl JwtVerifier {
    pub async fn new(project_id: String) -> Result<Self, JwtVerifierCreationError> {
        let public_keys = Self::fetch_public_keys().await?; // Err is `reqwest::Error`
        let decoding_keys = public_keys
            .into_iter()
            .map(|(key, value)| {
                DecodingKey::from_rsa_pem(value.as_bytes()).map(|decoding_key| (key, decoding_key))
            })
            .collect::<Result<HashMap<String, DecodingKey>,jsonwebtoken::errors::Error>>()?; // Err is `jsonwebtoken::errors::Error`
        unimplemented!()
    }

    async fn fetch_public_keys() -> Result<HashMap<String, String>, reqwest::Error> {
        unimplemented!()
    }
}

struct JwtVerifier;
```

## More Details

### Source Variants

Error sets that have source variants (aka wrapped variants), will delegate the `Error` trait's `source()` method to the
correct source branch's wrapped error. `From` traits are also automatically generated from the
inner type to the Error enum.

#### Source Tuple Variants
Source tuple variants are the most common source variant. Declared like
```rust
error_set::error_set! {
    ErrorEnum := {
        IoError(std::io::Error),
        FmtError(std::fmt::Error),
    }
}
```
Which has the generated enum
```rust
pub enum ErrorEnum {
    IoError(std::io::Error),
    FmtError(std::fmt::Error),
}
```

#### Source Struct Variants
Source struct variants are also supported, declared like so
```rust
error_set::error_set! {
    ErrorEnum := {
        IoError(std::io::Error) {} // Note the `{}`
    }
}
```
Which has the generated enum
```rust
pub enum ErrorEnum {
    IoError {
        source: std::io::Error,
    }
}
```
Source structs become useful when you want to attach additional fields to an error
```rust
error_set::error_set! {
    ErrorEnum := {
        IoError(std::io::Error) {
            field1: String,
            field2: &'static str,
        }
    }
}
```
Which has the generated enum
```rust
pub enum ErrorEnum {
    IoError {
        source: std::io::Error,
        field1: String,
        field2: &'static str,
    }
}
```
A `From` implementation for the inner `source` is not automatically generated for source struct variants that have fields,
like above.

#### Multiple Source Variants Of The Same Type
Error sets can have multiple source variants of the same type. e.g.
```rust
error_set::error_set! {
    ErrorEnum3 := {
        IoError1(std::io::Error),
        IoError2(std::io::Error),
    }
}
```
But a `From` implementation will not be automatically generated for these cases.

### Aggregations And Conversions

Error set uses `||` (or) for aggregation, which performs an "or" operation on the set space. 
Note, `||` is not needed, just a convenience -
```rust
error_set::error_set! {
    ErrorEnum1 := {
        Variant1,
        Variant2
    } || ErrorEnum2
    ErrorEnum2 := {
        Variant3
    }
}
```
is equivalent to
```rust
error_set::error_set! {
    ErrorEnum1 := {
        Variant1,
        Variant2,
        Variant3,
    }
    ErrorEnum2 := {
        Variant3
    }
}
```

For one type to be converted into another it needs to be considered a subset of the target type.
Thus in the example above, `ErrorEnum2` can be converted into `ErrorEnum1` with `.into()` or `?`.

### Display

The `#[display(...)]` attribute provides a custom display message for variant.
If a custom display is not provided for a wrapped error type like `IoError(std::io::Error)`, it will directly 
delegate its display to the inner type (`std::io::Error`). If it is desired to prevent this, provide a custom 
display message, like in the below example, or add `#[display(opaque)]`. The default display for other
variant types is `ErrorName::VariantName`.
```rust
error_set::error_set! {
    AuthError := {
        #[display("User `{name}` with role `{role}` does not exist")] // Shorthand for `#[display("User `{}` with role `{}` does not exist", name, role)]`
        UserDoesNotExist {
            name: String,
            role: u32,
        },
        #[display("The provided credentials are invalid")]
        InvalidCredentials
    }
    LoginError := {
        #[display("Io Error: {0}")] // Shorthand for `#[display("Io Error: {}", source)]`
        IoError(std::io::Error),
    } || AuthError
}
```

<details>

<summary>Usage</summary>

```rust
error_set::error_set! {
    AuthError := {
        #[display("User `{name}` with role `{role}` does not exist")] // Shorthand for `#[display("User `{}` with role `{}` does not exist", name, role)]`
        UserDoesNotExist {
            name: String,
            role: u32,
        },
        #[display("The provided credentials are invalid")]
        InvalidCredentials
    }
    LoginError := {
        #[display("Io Error: {0}")] // Shorthand for `#[display("Io Error: {}", source)]`
        IoError(std::io::Error),
    } || AuthError
}

fn main() {
    let x: AuthError = AuthError::UserDoesNotExist {
        name: "john".to_string(),
        role: 30,
    };
    assert_eq!(x.to_string(), "User `john` with role `30` does not exist".to_string());
    let y: LoginError = x.into();
    assert_eq!(y.to_string(), "User `john` with role `30` does not exist".to_string());
    let x = AuthError::InvalidCredentials;
    assert_eq!(x.to_string(), "The provided credentials are invalid".to_string());
}
```

</details>

Redeclaring the same variant in a different set and changing the display message, does not
effect the conversion between sets.

### Disable

error_set auto-implements `From`, `Display`, `Debug`, and `Error` for a set. If it is ever desired to disable
this. Add `#[disable(..)]` to the set. e.g.
```rust
use std::fmt::{Display, Debug};

error_set::error_set! {
    #[disable(Display,Debug)]
    X := {
        A,
    }
}

impl Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X")
    }
}

impl Debug for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X")
    }
}
```
`From` also accepts arguments to only disable certain `From` implementations. e.g.
```rust
error_set::error_set! {
    U := {
        IoError(std::io::Error),
    }
    V := {
        FmtError(std::fmt::Error),
        IoError(std::io::Error),
    }
    #[disable(From(std::io::Error, U))]
    W := V || U
}
```

### Generics

error_set supports generics. e.g.
```rust
use std::fmt::Debug;

error_set::error_set! {
    X<G: Debug> := {
        A {
            a: G
        }
    }
    Y<H: Debug> := {
        B {
            b: H
        }
    }
    Z<T: Debug> := X<T> || Y<T>
}
```
In `Z<T: Debug> := X<T> || Y<T>` `T` will replace `G` in `X` - `X<T: Debug>`. Thus this statement is the
same as writing
```rust
use std::fmt::Debug;

error_set::error_set! {
    // ...

    Z<T: Debug> := {
        A {
            a: T
        },
        B {
            b: T
        }
    }
}
```

### Why Choose `error_set` Over `thiserror` or `anyhow`

`error_set` is a unique approach with some of the same features of `thiserror` and `anyhow`, while solving a few more problems
common to Rust developers.

Like `thiserror`, `error_set` allows you define errors, their display messages, and conversions between errors. However `error_set`
is more maintainable and approximately 50% more concise:

<details>

<summary>example</summary>

```rust,ignore
// thiserror
#[derive(Error)]
enum Error1 {
    a,
    b,
}
#[derive(Error)]
enum Error2 {
    c,
    d,
}
#[derive(Error)]
enum Error3 {
    Error1(#[from] Error1),
    Error2(#[from] Error2),
}

// error_set
error_set! {
    Error1 := {
        a,
        b
    }
    Error2 := {
        c,
        d
    }
    Error3 := Error1 || Error2
    // `Error3` above is equivalent to writing
    // ```
    // Error3 = {
    //    a,
    //    b,
    //    c,
    //    d
    // };
    // ```
}
```

</details>

With `error_set` there is no need to maintain a web of nested wrapped enums (with `#[from]`), since there is no nesting, and all the `From` implementations are automatically generated if one error type is a subset of another.

Like `anyhow`, `error_set` remains open to capturing the context around errors. To accomplish this, it uses the help of [err_trail](https://github.com/mcmah309/error_set/tree/master/err_trail) or [eros](https://github.com/mcmah309/eros) crate. See the respective READMEs for more info. However, if your project doesn't require handling specific error types and you just need to propagate errors up the call stack, then `anyhow` is likely a good choice for you. It's straightforward and skips the need to define error types all together.

For libraries and general projects that require precise error handling and differentiation, error management can often become complex and unwieldy
as projects grow. This may even result in "mega enums". `error_set` can help here where others can't.

#### What is a Mega Enum?

A mega enum, or mega error enum, is an enumeration that consolidates various error types into one large enum, whereas the code would be more precise if split into multiple enums.
These often arise due to refactors or developers opting for less intrusive programming approach.
This method can lead to inefficiencies and confusion because it includes error variants that are not relevant in certain scopes. 

##### Example Scenario:

Consider the following functions and their respective error types:

- `func1` can produce errors `a` and `b`, represented by `enum1`.
- `func2` can produce errors `c` and `d`, represented by `enum2`.
- `func3` calls both `func1` and `func2`.

If `func3` does not handle the errors from `func1` and `func2`, it must return an error enum that encompasses variants `a`, `b`, `c`, and `d`. Without a tool like `error_set`, developers might skip defining `enum1` and `enum2` due to the complexity and instead create a mega enum with all possible error variants (`a`, `b`, `c`, `d`). This means that any caller of `func1` or `func2` would have to handle all these cases, even those that are not possible in that specific context. `error_set` being so concise and simple, developers actually want to scope their errors to the correct context and join them when needed with a simple `||` operation. No need to ever think about a web of nested wrapped error types.

##### How `error_set` Simplifies Error Management:

`error_set` allows you to define errors quickly and precisely. Correctly scoping errors is easy and no wrapping of
various error enum types is necessary. Conversions/Propagation up the stack are as simple as `.into()` or `?`.
`error_set` also makes display messages and tracking context easy.
By using `error_set`, your project can maintain clear and precise error definitions, enhancing code readability and maintainability without the tedious process of manually defining and managing error relations.

### no_std

This crate supports `#![no_std]`. 

Cavets:
 - `tracing`/`log` features are not supported, but `defmt` is supported.