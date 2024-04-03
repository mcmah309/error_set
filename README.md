# error_set

## Idea
Rust implementation of zig's [error set](https://ziglang.org/documentation/master/#Error-Set-Type)
to concisely define error types and convert between.

Instead of defining various enums for errors. Use an error set.
```rust
error_set!( SetLevelError,{
    MagazineParsingError {
        MissingNameArg,
        MissingPublishTimeArg
    },
    BookParsingError {
        MissingNameArg,
        MissingPublishTimeArg,
        MissingDescriptionArg,
    },
})
```
Output:
```rust
#[derive(Debug)]
enum SetLevelError {
    MissingNameArg,
    MissingPublishTimeArg,
    MissingDescriptionArg,
}
#[derive(Debug)]
enum MagazineParsingError {
    MissingNameArg,
    MissingPublishTimeArg,
}
#[derive(Debug)]
enum BookParsingError {
    MissingNameArg,
    MissingPublishTimeArg,
    MissingDescriptionArg,
}

impl std::error::Error for SetLevelError {}
impl std::error::Error for MagazineParsingError {}
impl std::error::Error for BookParsingError {}

impl From<MagazineParsingError> for SetLevelError {
    fn from(error: MagazineParsingError) -> Self {
        match error {
            MagazineParsingError::MissingNameArg => SetLevelError::MissingNameArg,
            MagazineParsingError::MissingPublishTimeArg => SetLevelError::MissingPublishTimeArg,
        }
    }
}
impl From<BookParsingError> for SetLevelError {
    fn from(error: BookParsingError) -> Self {
        match error {
            BookParsingError::MissingNameArg => SetLevelError::MissingNameArg,
            BookParsingError::MissingPublishTimeArg => SetLevelError::MissingPublishTimeArg,
            BookParsingError::MissingDescriptionArg => SetLevelError::MissingDescriptionArg,
        }
    }
}


impl From<MagazineParsingError> for BookParsingError {
    fn from(error: MagazineParsingError) -> Self {
        match error {
            MagazineParsingError::MissingNameArg => BookParsingError::MissingNameArg,
            MagazineParsingError::MissingPublishTimeArg => BookParsingError::MissingPublishTimeArg,
        }
    }
}

impl core::fmt::Display for SetLevelError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            SetLevelError::MissingNameArg => "SetLevelError::MissingNameArg",
            SetLevelError::MissingPublishTimeArg => "SetLevelError::MissingPublishTimeArg",
            SetLevelError::MissingDescriptionArg => "SetLevelError::MissingDescriptionArg",
        };
        write!(f, "{}", variant_name)
    }
}
impl core::fmt::Display for MagazineParsingError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            MagazineParsingError::MissingNameArg => "MagazineParsingError::MissingNameArg",
            MagazineParsingError::MissingPublishTimeArg => "MagazineParsingError::MissingPublishTimeArg",
        };
        write!(f, "{}", variant_name)
    }
}
impl core::fmt::Display for BookParsingError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let variant_name = match *self {
            BookParsingError::MissingNameArg => "BookParsingError::MissingNameArg",
            BookParsingError::MissingPublishTimeArg => "BookParsingError::MissingPublishTimeArg",
            BookParsingError::MissingDescriptionArg => "BookParsingError::MissingDescriptionArg",
        };
        write!(f, "{}", variant_name)
    }
}
```
Usage
```rust
fn main() {
    let magazine_error = MagazineParsingError::MissingNameArg;
    let crate_error: SetLevelError = magazine_error.into();
    println!("{:?}", crate_error);

    let book_error = BookParsingError::MissingDescriptionArg;
    let crate_error_from_book: SetLevelError = book_error.into();
    println!("{:?}", crate_error_from_book);
}
```