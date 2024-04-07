# error_set

[<img alt="github" src="https://img.shields.io/badge/github-mcmah309/error_set-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mcmah309/error_set)
[<img alt="crates.io" src="https://img.shields.io/crates/v/error_set.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/error_set)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-error_set-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/error_set)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/mcmah309/error_set/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/mcmah309/error_set/actions?query=branch%3Amaster)


A concise way to define errors and ergomically coerce a subset into a superset with with just `.into()`, or `?`.

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
<details>

  <summary>Cargo Expand</summary>

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use error_set::error_set;
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
impl error_set::ErrorSetMarker for MediaError {}
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
impl error_set::ErrorSetMarker for BookParsingError {}
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
impl error_set::ErrorSetMarker for BookSectionParsingError {}
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
impl error_set::ErrorSetMarker for DownloadError {}
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
impl error_set::ErrorSetMarker for UploadError {}
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

which is also equivalent to writing:
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
Error enums and error variants can also accept doc comments and attributes like `#[derive(...)]`.

### Examples
<details>

  <summary>Base Functionality In Action</summary>

```rust
fn main() {
        let book_section_parsing_error = BookSectionParsingError::MissingNameArg;
        let book_parsing_error: BookParsingError = book_section_parsing_error.coerce(); // `.coerce()` == `.into()`
        assert!(matches!(book_parsing_error, BookParsingError::MissingNameArg));
        let media_error: MediaError = book_parsing_error.coerce(); // `.coerce()` == `.into()`
        assert!(matches!(media_error, MediaError::MissingNameArg));

        let io_error = std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory");
        let result_download_error: Result<(), DownloadError> = Err(io_error).coerce(); // `.coerce()` == `.map_err(Into::into)`
        let result_media_error: Result<(), MediaError> = result_download_error.coerce(); // `.coerce()` == `.map_err(Into::into)`
        assert!(matches!(result_media_error, Err(MediaError::IoError(_))));
}
```
</details>

<details>
Here we can easily define all the different error states a function could exit with. Note this example is verbose as not all error states have downstream handlers that care about the error type, but imagine it so.
  <summary>More Intricate Example</summary>

```rust
error_set::error_set! {
    MediaError = {
        IoError(std::io::Error)
    } || BookParsingError || DownloadError || UploadError;
    BookParsingError = {
        MissingContent,
        BookAccess(std::io::Error),
    };
    DownloadError = {
        InvalidUrl,
        CouldNotSaveBook(std::io::Error),
    };
    UploadError = {
        MaximumUploadSizeReached,
        TimedOut,
        AuthenticationFailed,
    };
}

fn parse_book(file_path: &str) -> Result<String, BookParsingError> {
    let mut file = File::open(file_path).coerce::<BookParsingError>()?;
    let mut content = String::new();
    file.read_to_string(&mut content).coerce::<BookParsingError>()?;
    if content.is_empty() {
        Err(BookParsingError::MissingContent)
    } else {
        Ok(content)
    }
}

fn download_book(url: &str, save_path: &str) -> Result<(), DownloadError> {
    if url.is_empty() {
        Err(DownloadError::InvalidUrl)
    } else {
        let simulated_book_content = "This is a downloaded book content.";
        let mut file = File::create(save_path).coerce::<DownloadError>()?;
        file.write_all(simulated_book_content.as_bytes()).coerce::<DownloadError>()?;
        Ok(())
    }
}

// Simulated function to upload the book content
fn upload_book(content: &str) -> Result<(), UploadError> {
    let auth = true;
    if !auth { // Simulate auth
        return Err(UploadError::AuthenticationFailed);
    }
    let time_out = false;
    if !time_out { // Simulate timeout uploading
        return Err(UploadError::TimedOut);
    }
    if content.len() > 1024 { // Simulate an upload size limit
        Err(UploadError::MaximumUploadSizeReached)
    } else {
        println!("Book uploaded successfully.");
        Ok(())
    }
}

fn process_book(download_path: &str, download_url: &str) -> Result<String, MediaError> {
    download_book(download_url, download_path).coerce::<MediaError>()?;
    let content = parse_book(download_path).coerce::<MediaError>()?;
    const MAX_RETRIES: u8  = 3;
    let mut current_retries = 0;
    match upload_book(&content) {
        Err(UploadError::TimedOut) => {
            while current_retries < MAX_RETRIES {
                current_retries += 1;
                if let Ok(_) = upload_book(&content) {
                    break;
                }
            }
        }
        Err(e) => return Err(e.coerce()),
        _ => (),
    }
    fs::remove_file(download_path).coerce::<MediaError>()?;
    Ok(content)
}

fn main() {
    match process_book("downloaded_book.txt", "http://example.com/book") {
        Ok(content) => println!("Book processed successfully: {}", content),
        Err(e) => eprintln!("An error occurred: {:?}", e),
    }
}
```
</details>