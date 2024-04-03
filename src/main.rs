use std::io::Error;

fn main() {
    let x: Result<(), MagazineParsingError> = Err(MagazineParsingError::MissingNameArg);
    let y: Result<(), BookParsingError> = x.map_err(Into::into);
    println!("Hello, world!");
}

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
// trait Coerce<U> {
//     fn coerce(self: Self) -> U;
// }

// impl<T> Coerce<Result<T,BookParsingError>> for Result<T,MagazineParsingError> {
//     fn coerce(self: Self) -> Result<T,BookParsingError> {
//         self.map_err(|error| error.into())
//     }
// }

// trait IntoResultBookParsingError<T> {
//     fn into(self) -> Result<T,BookParsingError>;
// }

// impl<T> IntoResultBookParsingError<T> for Result<T,MagazineParsingError> {
//     fn into(self) -> Result<T,BookParsingError> {
//         self.map_err(|error| error.into())
//     }
// }

// impl<T> From<Result<T,MagazineParsingError>> for Result<T,BookParsingError> {
//     fn from(result: Result<T,MagazineParsingError>) -> Self {
//         result.map_err(|error| error.into())
//     }
// }

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