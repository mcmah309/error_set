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
    MissingNameArg,
    NoContents,
    MissingDescriptionArg,
    CouldNotConnect,
    IoError(std::io::Error),
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
            MediaError::MissingNameArg => "MediaError::MissingNameArg",
            MediaError::NoContents => "MediaError::NoContents",
            MediaError::MissingDescriptionArg => "MediaError::MissingDescriptionArg",
            MediaError::CouldNotConnect => "MediaError::CouldNotConnect",
            MediaError::IoError(_) => "MediaError::IoError",
        };
        f.write_fmt(format_args!("{0}", variant_name))
    }
}
impl core::fmt::Debug for MediaError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            MediaError::MissingNameArg => "MediaError::MissingNameArg",
            MediaError::NoContents => "MediaError::NoContents",
            MediaError::MissingDescriptionArg => "MediaError::MissingDescriptionArg",
            MediaError::CouldNotConnect => "MediaError::CouldNotConnect",
            MediaError::IoError(_) => "MediaError::IoError",
        };
        f.write_fmt(format_args!("{0}", variant_name))
    }
}
impl From<BookParsingError> for MediaError {
    fn from(error: BookParsingError) -> Self {
        match error {
            BookParsingError::MissingNameArg => MediaError::MissingNameArg,
            BookParsingError::NoContents => MediaError::NoContents,
            BookParsingError::MissingDescriptionArg => MediaError::MissingDescriptionArg,
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
    MissingNameArg,
    NoContents,
    MissingDescriptionArg,
}
#[allow(unused_qualifications)]
impl std::error::Error for BookParsingError {}
impl core::fmt::Display for BookParsingError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            BookParsingError::MissingNameArg => "BookParsingError::MissingNameArg",
            BookParsingError::NoContents => "BookParsingError::NoContents",
            BookParsingError::MissingDescriptionArg => {
                "BookParsingError::MissingDescriptionArg"
            }
        };
        f.write_fmt(format_args!("{0}", variant_name))
    }
}
impl core::fmt::Debug for BookParsingError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            BookParsingError::MissingNameArg => "BookParsingError::MissingNameArg",
            BookParsingError::NoContents => "BookParsingError::NoContents",
            BookParsingError::MissingDescriptionArg => {
                "BookParsingError::MissingDescriptionArg"
            }
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
impl core::fmt::Debug for BookSectionParsingError {
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
impl core::fmt::Debug for DownloadError {
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
impl core::fmt::Debug for UploadError {
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