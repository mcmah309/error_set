#![no_std]

use error_set::{error_set, CoerceResult};

fn main() {
    readme_example();
}

//************************************************************************//

error_set! {
    MediaError = {
        IoError(self::TestError),
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
        CouldNotReadBook(self::TestError),
        MissingName,
        NoContents,
    };
    BookSectionParsingError = {
        MissingName,
        NoContents,
    };
    DownloadError = {
        InvalidUrl,
        CouldNotSaveBook(self::TestError),
    };
    ParseUploadError = {
        MaximumUploadSizeReached,
        TimedOut,
        AuthenticationFailed,
    };
}

struct TestError(u32);

impl TestError {
    pub fn new(code: u32) -> Self {
        Self(code)
    }
}

impl core::error::Error for TestError {}

impl core::fmt::Display for TestError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "TestError: {}", self.0)
    }
}

impl core::fmt::Debug for TestError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "TestError: {}", self.0)
    }
}

fn readme_example() {
    let book_section_parsing_error: BookSectionParsingError =
    BookSectionParsingError::MissingName;
    let book_parsing_error: BookParsingError = book_section_parsing_error.into();
    assert!(matches!(book_parsing_error, BookParsingError::MissingName));
    let media_error: MediaError = book_parsing_error.into();
    assert!(matches!(media_error, MediaError::MissingName));

    let io_error = TestError::new(500);
    let result_download_error: Result<(), DownloadError> = Err(io_error).coerce();
    let result_media_error: Result<(), MediaError> = result_download_error.map_err(Into::into);
    assert!(matches!(result_media_error, Err(MediaError::IoError(_))));
}