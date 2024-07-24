# Error Set

[<img alt="github" src="https://img.shields.io/badge/github-mcmah309/error_set-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mcmah309/error_set)
[<img alt="crates.io" src="https://img.shields.io/crates/v/error_set.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/error_set)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-error_set-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/error_set)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/mcmah309/error_set/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/mcmah309/error_set/actions?query=branch%3Amaster)

Error Set simplifies error management by providing a streamlined method for defining errors and easily converting between them. Resultingly, error handling becomes both straightforward and efficient.

Error Set is inspired by Zig's [error set](https://ziglang.org/documentation/master/#Error-Set-Type), and offers similar functionality.

Instead of defining various enums/structs for errors and hand rolling relations, use an error set:
```rust
use error_set::error_set;

error_set! {
    MediaError = BookParsingError || DownloadError || ParseUploadError;
    BookParsingError = {
        MissingBookDescription,
        IoError(std::io::Error),
    } || BookSectionParsingError;
    BookSectionParsingError = {
        MissingName,
        NoContents,
    };
    DownloadError = {
        InvalidUrl,
        IoError(std::io::Error),
    };
    ParseUploadError = {
        MaximumUploadSizeReached,
        TimedOut,
        AuthenticationFailed,
    };
}
```
<details>

  <summary>Cargo Expand</summary>

```rust
#[derive(Debug)]
pub enum MediaError {
    MissingBookDescription,
    IoError(std::io::Error),
    MissingName,
    NoContents,
    InvalidUrl,
    MaximumUploadSizeReached,
    TimedOut,
    AuthenticationFailed,
}
#[allow(unused_qualifications)]
impl std::error::Error for MediaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            MediaError::IoError(ref source) => source.source(),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
impl core::fmt::Display for MediaError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            MediaError::MissingBookDescription => "MediaError::MissingBookDescription",
            MediaError::IoError(_) => "MediaError::IoError",
            MediaError::MissingName => "MediaError::MissingName",
            MediaError::NoContents => "MediaError::NoContents",
            MediaError::InvalidUrl => "MediaError::InvalidUrl",
            MediaError::MaximumUploadSizeReached => "MediaError::MaximumUploadSizeReached",
            MediaError::TimedOut => "MediaError::TimedOut",
            MediaError::AuthenticationFailed => "MediaError::AuthenticationFailed",
        };
        f.write_fmt($crate::format_args!("{}", variant_name))
    }
}
impl From<BookParsingError> for MediaError {
    fn from(error: BookParsingError) -> Self {
        match error {
            BookParsingError::MissingBookDescription => MediaError::MissingBookDescription,
            BookParsingError::IoError(source) => MediaError::IoError(source),
            BookParsingError::MissingName => MediaError::MissingName,
            BookParsingError::NoContents => MediaError::NoContents,
        }
    }
}
impl From<BookSectionParsingError> for MediaError {
    fn from(error: BookSectionParsingError) -> Self {
        match error {
            BookSectionParsingError::MissingName => MediaError::MissingName,
            BookSectionParsingError::NoContents => MediaError::NoContents,
        }
    }
}
impl From<DownloadError> for MediaError {
    fn from(error: DownloadError) -> Self {
        match error {
            DownloadError::InvalidUrl => MediaError::InvalidUrl,
            DownloadError::IoError(source) => MediaError::IoError(source),
        }
    }
}
impl From<ParseUploadError> for MediaError {
    fn from(error: ParseUploadError) -> Self {
        match error {
            ParseUploadError::MaximumUploadSizeReached => MediaError::MaximumUploadSizeReached,
            ParseUploadError::TimedOut => MediaError::TimedOut,
            ParseUploadError::AuthenticationFailed => MediaError::AuthenticationFailed,
        }
    }
}
impl From<std::io::Error> for MediaError {
    fn from(error: std::io::Error) -> Self {
        MediaError::IoError(error)
    }
}
#[derive(Debug)]
pub enum BookParsingError {
    MissingBookDescription,
    IoError(std::io::Error),
    MissingName,
    NoContents,
}
#[allow(unused_qualifications)]
impl std::error::Error for BookParsingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            BookParsingError::IoError(ref source) => source.source(),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
impl core::fmt::Display for BookParsingError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            BookParsingError::MissingBookDescription => "BookParsingError::MissingBookDescription",
            BookParsingError::IoError(_) => "BookParsingError::IoError",
            BookParsingError::MissingName => "BookParsingError::MissingName",
            BookParsingError::NoContents => "BookParsingError::NoContents",
        };
        f.write_fmt($crate::format_args!("{}", variant_name))
    }
}
impl From<BookSectionParsingError> for BookParsingError {
    fn from(error: BookSectionParsingError) -> Self {
        match error {
            BookSectionParsingError::MissingName => BookParsingError::MissingName,
            BookSectionParsingError::NoContents => BookParsingError::NoContents,
        }
    }
}
impl From<std::io::Error> for BookParsingError {
    fn from(error: std::io::Error) -> Self {
        BookParsingError::IoError(error)
    }
}
#[derive(Debug)]
pub enum BookSectionParsingError {
    MissingName,
    NoContents,
}
#[allow(unused_qualifications)]
impl std::error::Error for BookSectionParsingError {}

impl core::fmt::Display for BookSectionParsingError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            BookSectionParsingError::MissingName => "BookSectionParsingError::MissingName",
            BookSectionParsingError::NoContents => "BookSectionParsingError::NoContents",
        };
        f.write_fmt($crate::format_args!("{}", variant_name))
    }
}
#[derive(Debug)]
pub enum DownloadError {
    InvalidUrl,
    IoError(std::io::Error),
}
#[allow(unused_qualifications)]
impl std::error::Error for DownloadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            DownloadError::IoError(ref source) => source.source(),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
impl core::fmt::Display for DownloadError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            DownloadError::InvalidUrl => "DownloadError::InvalidUrl",
            DownloadError::IoError(_) => "DownloadError::IoError",
        };
        f.write_fmt($crate::format_args!("{}", variant_name))
    }
}
impl From<std::io::Error> for DownloadError {
    fn from(error: std::io::Error) -> Self {
        DownloadError::IoError(error)
    }
}
#[derive(Debug)]
pub enum ParseUploadError {
    MaximumUploadSizeReached,
    TimedOut,
    AuthenticationFailed,
}
#[allow(unused_qualifications)]
impl std::error::Error for ParseUploadError {}

impl core::fmt::Display for ParseUploadError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            ParseUploadError::MaximumUploadSizeReached => {
                "ParseUploadError::MaximumUploadSizeReached"
            }
            ParseUploadError::TimedOut => "ParseUploadError::TimedOut",
            ParseUploadError::AuthenticationFailed => "ParseUploadError::AuthenticationFailed",
        };
        f.write_fmt($crate::format_args!("{}", variant_name))
    }
}
```
</details>

which is also equivalent to writing the full expansion:
```rust
error_set! {
    MediaError = {
        IoError(std::io::Error),
        MissingBookDescription,
        MissingName,
        NoContents,
        InvalidUrl,
        MaximumUploadSizeReached,
        TimedOut,
        AuthenticationFailed,
    };
    BookParsingError = {
        MissingBookDescription,
        IoError(std::io::Error),
        MissingName,
        NoContents,
    };
    BookSectionParsingError = {
        MissingName,
        NoContents,
    };
    DownloadError = {
        InvalidUrl,
        IoError(std::io::Error),
    };
    ParseUploadError = {
        MaximumUploadSizeReached,
        TimedOut,
        AuthenticationFailed,
    };
}
```
Any above subset can be converted into a superset with `.into()` or `?`. 
This makes correctly scoping and passing around errors a breeze.
Error enums and error variants can also accept doc comments and attributes like `#[derive(...)]`.
The typical project approach is to have one `errors.rs` file with a single `error_set`. This keeps
all the errors in one place and allows your IDE to autocomplete `crate::errors::` with of all errors.

<details>

  <summary>Basic Example</summary>

```rust
use error_set::error_set;

error_set! {
    MediaError = {
        IoError(std::io::Error)
    } || BookParsingError || DownloadError || ParseUploadError;
    BookParsingError = {
        MissingBookDescription,
        CouldNotReadBook(std::io::Error),
    } || BookSectionParsingError;
    BookSectionParsingError = {
        MissingName,
        NoContents,
    };
    DownloadError = {
        InvalidUrl,
        CouldNotSaveBook(std::io::Error),
    };
    ParseUploadError = {
        MaximumUploadSizeReached,
        TimedOut,
        AuthenticationFailed,
    };
}

fn main() {
    let book_section_parsing_error: BookSectionParsingError = BookSectionParsingError::MissingName;
    let book_parsing_error: BookParsingError = book_section_parsing_error.into();
    assert!(matches!(book_parsing_error, BookParsingError::MissingName));
    let media_error: MediaError = book_parsing_error.into();
    assert!(matches!(media_error, MediaError::MissingName));

    let io_error = std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory");
    let result_download_error: Result<(), DownloadError> = Err(io_error).coerce(); // `.coerce()` == `.map_err(Into::into)`
    let result_media_error: Result<(), MediaError> = result_download_error.coerce(); // `.coerce()` == `.map_err(Into::into)`
    assert!(matches!(result_media_error, Err(MediaError::IoError(_))));
}
```
</details>

### Feature Flags

**coerce_macro:** Each error set will generates a `coerce!` macro to help handle coercion between partially intersecting sets.

```rust
let val = coerce!(setx => {
                    Ok(val) => val,
                    Err(SetX::X) => {}, // handle disjointedness
                    { Err(SetX) => return Err(SetY) } // terminal coercion
                })?;
```

<details>

<summary>More Details</summary>

Given:
 ```rust
 error_set! {
    SetX = {
        X
    } || Common;
    SetY = {
        Y
    } || Common;
    Common = {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
    };
 }
 ```

 rather than writing:

 ```rust
 fn setx_result_to_sety_result() -> Result<(), SetY> {
    let _ok = match setx_result() {
        Ok(ok) => ok,
        Err(SetX::X) => {} // handle disjointedness
        Err(SetX::A) => {
            return Err(SetY::A);
        }
        Err(SetX::B) => {
            return Err(SetY::B);
        }
        Err(SetX::C) => {
            return Err(SetY::C);
        }
        Err(SetX::D) => {
            return Err(SetY::D);
        }
        Err(SetX::E) => {
            return Err(SetY::E);
        }
        Err(SetX::F) => {
            return Err(SetY::F);
        }
        Err(SetX::G) => {
            return Err(SetY::G);
        }
        Err(SetX::H) => {
            return Err(SetY::H);
        }
    };
    Ok(())
 }
 ```

 one can write this, which compiles to the `match` statement above:

 ```rust
 fn setx_result_to_sety_result() -> Result<(), SetY> {
    let _ok = coerce!(setx_result() => {
        Ok(ok) => ok,
        Err(SetX::X) => {}, // handle disjointedness
        { Err(SetX) => return Err(SetY) } // terminal coercion
    });
    Ok(())
 }
 ```

 The `coerce!` macro is a flat fast (no tt muncher 🦫) declarative macro created by the `error_set!` macro for the set.
 `coerce!` behaves like a regular `match` statement, except it allows a terminal coercion statement between sets. e.g.

 ```rust
 { Err(SetX) => return Err(SetY) }
 { Err(SetX) => Err(SetY) }
 { SetX => return SetY }
 { SetX => SetY }
 ```

 With `coerce!`, one can concisely handle specific variants of errors as they bubble up the call stack and propagate the rest.
</details>

**tracing:** Enables support for the tracing crate. Adds methods to `Result` that are applied on `Err`. Similar to anyhow's `.context(..)`.
```rust
let value = result.warn("This a warning that will be passed to tracing if `Err`")?;
```
**log:** Enables support for the log crate. Adds methods to `Result` that are applied on `Err`. Similar to anyhow's `.context(..)`.
```rust
let value = result.warn("This a warning that will be passed to log if `Err`")?;
```

### Why Choose `error_set` Over `thiserror` or `anyhow`

If your project doesn't require handling specific error types and you just need to propagate errors up the call stack, then `anyhow` is likely a good choice for you. It's straightforward and skips the need to define error types all together.

However, for libraries and general projects that require precise error handling and differentiation, error management can often become complex and unwieldy, especially if "mega enums" arise. 

**What is a Mega Enum?**

A mega enum, or mega error enum, is an enumeration that consolidates various error types into one large enum, whereas the code would be more precise if split into multiple enums.
These often arise due to refactors or developers opting for less intrusive programming approach.
This method can lead to inefficiencies and confusion because it includes error variants that are not relevant in certain scopes. 

**Example Scenario:**

Consider the following functions and their respective error types:

- `func1` can produce errors `a` and `b`, represented by `enum1`.
- `func2` can produce errors `c` and `d`, represented by `enum2`.
- `func3` calls both `func1` and `func2`.

If `func3` does not handle the errors from `func1` and `func2`, it must return an error enum that encompasses variants `a`, `b`, `c`, and `d`. Without a tool like `error_set`, developers might skip defining `enum1` and `enum2` due to the complexity and instead create a mega enum with all possible error variants (`a`, `b`, `c`, `d`). This means that any caller of `func1` or `func2` would have to handle all these cases, even those that are not possible in that specific context.

**How `error_set` Simplifies Error Management:**

`error_set` allows you to define errors quickly and precisely. Correctly scoping errors is easy and no wrapping of
various error enum types is necessary, just use `.into()` or `?` (or `coerce!` macro).
This approach ensures that each function only deals with relevant error variants, avoiding the clutter and inefficiency of mega enums. 
By using `error_set`, your project can maintain clear and precise error definitions, enhancing code readability and maintainability without the tedious process of manually defining and managing error relations.