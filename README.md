# error_set

A concise way to define errors and ergomically coerce a subset to a superset with with just `.into()`.

`error_set` was inspired by zig's [error set](https://ziglang.org/documentation/master/#Error-Set-Type)
and works functionally the same.

Instead of defining various enums/structs for errors and hand rolling relations, use an error set:
```rust
use error_set::error_set;

error_set! {
    MediaError = {
        IoError(std::io::Error)
        } || BookParsingError || DownloadError || UploadError;
    BookParsingError = {
        MissingDescriptionArg
    } || BookSectionParsingError;
    BookSectionParsingError = {
        MissingNameArg,
        NoContents,
    };
    DownloadError = {
        CouldNotConnect,
        OutOfMemory(std::io::Error),
    };
    UploadError = {
        NoConnection(std::io::Error),
    };
}
```
which is equivlent to writing:
```rust
error_set! {
    MediaError = {
        MissingNameArg,
        NoContents,
        MissingDescriptionArg,
        CouldNotConnect,
        IoError(std::io::Error),
    };
    BookParsingError = {
        MissingNameArg,
        NoContents,
        MissingDescriptionArg,
    };
    BookSectionParsingError = {
        MissingNameArg,
        NoContents,
    };
    DownloadError = {
        CouldNotConnect,
        OutOfMemory(std::io::Error),
    };
    UploadError = {
        NoConnection(std::io::Error),
    };
}
```
Usage
```rust
fn main() {
        let book_section_parsing_error = BookSectionParsingError::MissingNameArg;
        let book_parsing_error: BookParsingError = book_section_parsing_error.into();
        assert!(matches!(
            book_parsing_error,
            BookParsingError::MissingNameArg
        ));
        let media_error: MediaError = book_parsing_error.into();
        assert!(matches!(media_error, MediaError::MissingNameArg));

        let io_error =std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory");
        let result_download_error: Result<(), DownloadError> = Err(io_error).map_err(Into::into);
        let result_media_error: Result<(), MediaError> = result_download_error.map_err(Into::into);
        assert!(matches!(result_media_error, Err(MediaError::IoError(_))));
}
```
<details>

  <summary>Cargo Expand</summary>

```rust
pub enum MediaError {
    IoError(std::io::Error),
    MissingDescriptionArg,
    MissingNameArg,
    NoContents,
    CouldNotConnect,
}
#[automatically_derived]
impl ::core::fmt::Debug for MediaError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            MediaError::IoError(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "IoError",
                    &__self_0,
                )
            }
            MediaError::MissingDescriptionArg => {
                ::core::fmt::Formatter::write_str(f, "MissingDescriptionArg")
            }
            MediaError::MissingNameArg => {
                ::core::fmt::Formatter::write_str(f, "MissingNameArg")
            }
            MediaError::NoContents => ::core::fmt::Formatter::write_str(f, "NoContents"),
            MediaError::CouldNotConnect => {
                ::core::fmt::Formatter::write_str(f, "CouldNotConnect")
            }
        }
    }
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
            MediaError::IoError(_) => "MediaError::IoError",
            MediaError::MissingDescriptionArg => "MediaError::MissingDescriptionArg",
            MediaError::MissingNameArg => "MediaError::MissingNameArg",
            MediaError::NoContents => "MediaError::NoContents",
            MediaError::CouldNotConnect => "MediaError::CouldNotConnect",
        };
        f.write_fmt(format_args!("{0}", variant_name))
    }
}
impl From<BookParsingError> for MediaError {
    fn from(error: BookParsingError) -> Self {
        match error {
            BookParsingError::MissingDescriptionArg => MediaError::MissingDescriptionArg,
            BookParsingError::MissingNameArg => MediaError::MissingNameArg,
            BookParsingError::NoContents => MediaError::NoContents,
        }
    }
}
impl From<BookSectionParsingError> for MediaError {
    fn from(error: BookSectionParsingError) -> Self {
        match error {
            BookSectionParsingError::MissingNameArg => MediaError::MissingNameArg,
            BookSectionParsingError::NoContents => MediaError::NoContents,
        }
    }
}
impl From<DownloadError> for MediaError {
    fn from(error: DownloadError) -> Self {
        match error {
            DownloadError::CouldNotConnect => MediaError::CouldNotConnect,
            DownloadError::OutOfMemory(source) => MediaError::IoError(source),
        }
    }
}
impl From<UploadError> for MediaError {
    fn from(error: UploadError) -> Self {
        match error {
            UploadError::NoConnection(source) => MediaError::IoError(source),
        }
    }
}
impl From<std::io::Error> for MediaError {
    fn from(error: std::io::Error) -> Self {
        MediaError::IoError(error)
    }
}
pub enum BookParsingError {
    MissingDescriptionArg,
    MissingNameArg,
    NoContents,
}
#[automatically_derived]
impl ::core::fmt::Debug for BookParsingError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                BookParsingError::MissingDescriptionArg => "MissingDescriptionArg",
                BookParsingError::MissingNameArg => "MissingNameArg",
                BookParsingError::NoContents => "NoContents",
            },
        )
    }
}
#[allow(unused_qualifications)]
impl std::error::Error for BookParsingError {}
impl core::fmt::Display for BookParsingError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            BookParsingError::MissingDescriptionArg => {
                "BookParsingError::MissingDescriptionArg"
            }
            BookParsingError::MissingNameArg => "BookParsingError::MissingNameArg",
            BookParsingError::NoContents => "BookParsingError::NoContents",
        };
        f.write_fmt(format_args!("{0}", variant_name))
    }
}
impl From<BookSectionParsingError> for BookParsingError {
    fn from(error: BookSectionParsingError) -> Self {
        match error {
            BookSectionParsingError::MissingNameArg => BookParsingError::MissingNameArg,
            BookSectionParsingError::NoContents => BookParsingError::NoContents,
        }
    }
}
pub enum BookSectionParsingError {
    MissingNameArg,
    NoContents,
}
#[automatically_derived]
impl ::core::fmt::Debug for BookSectionParsingError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                BookSectionParsingError::MissingNameArg => "MissingNameArg",
                BookSectionParsingError::NoContents => "NoContents",
            },
        )
    }
}
#[allow(unused_qualifications)]
impl std::error::Error for BookSectionParsingError {}
impl core::fmt::Display for BookSectionParsingError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            BookSectionParsingError::MissingNameArg => {
                "BookSectionParsingError::MissingNameArg"
            }
            BookSectionParsingError::NoContents => "BookSectionParsingError::NoContents",
        };
        f.write_fmt(format_args!("{0}", variant_name))
    }
}
pub enum DownloadError {
    CouldNotConnect,
    OutOfMemory(std::io::Error),
}
#[automatically_derived]
impl ::core::fmt::Debug for DownloadError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DownloadError::CouldNotConnect => {
                ::core::fmt::Formatter::write_str(f, "CouldNotConnect")
            }
            DownloadError::OutOfMemory(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "OutOfMemory",
                    &__self_0,
                )
            }
        }
    }
}
#[allow(unused_qualifications)]
impl std::error::Error for DownloadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            DownloadError::OutOfMemory(ref source) => source.source(),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
impl core::fmt::Display for DownloadError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            DownloadError::CouldNotConnect => "DownloadError::CouldNotConnect",
            DownloadError::OutOfMemory(_) => "DownloadError::OutOfMemory",
        };
        f.write_fmt(format_args!("{0}", variant_name))
    }
}
impl From<UploadError> for DownloadError {
    fn from(error: UploadError) -> Self {
        match error {
            UploadError::NoConnection(source) => DownloadError::OutOfMemory(source),
        }
    }
}
impl From<std::io::Error> for DownloadError {
    fn from(error: std::io::Error) -> Self {
        DownloadError::OutOfMemory(error)
    }
}
pub enum UploadError {
    NoConnection(std::io::Error),
}
#[automatically_derived]
impl ::core::fmt::Debug for UploadError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            UploadError::NoConnection(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "NoConnection",
                    &__self_0,
                )
            }
        }
    }
}
#[allow(unused_qualifications)]
impl std::error::Error for UploadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            UploadError::NoConnection(ref source) => source.source(),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
impl core::fmt::Display for UploadError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            UploadError::NoConnection(_) => "UploadError::NoConnection",
        };
        f.write_fmt(format_args!("{0}", variant_name))
    }
}
impl From<std::io::Error> for UploadError {
    fn from(error: std::io::Error) -> Self {
        UploadError::NoConnection(error)
    }
}
```
</details>