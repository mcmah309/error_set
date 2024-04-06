#[cfg(test)]
pub mod regular {
    use error_set::error_set;

    error_set! {
        SetLevelError = {
            MissingNameArg,
            MissingPublishTimeArg,
            MissingDescriptionArg,
        };
        MagazineParsingError = {
            MissingNameArg,
        };
        BookParsingError = {
            MissingNameArg,
            MissingPublishTimeArg,
            MissingDescriptionArg,
        };
    }

    #[test]
    fn into_works_correctly() {
        let magazine_error = MagazineParsingError::MissingNameArg;
        let crate_error: SetLevelError = magazine_error.into();
        println!("{:?}", crate_error);

        let book_error = BookParsingError::MissingDescriptionArg;
        let crate_error_from_book: SetLevelError = book_error.into();
        println!("{:?}", crate_error_from_book);

        let x: Result<(), MagazineParsingError> = Err(MagazineParsingError::MissingNameArg);
        let _y: Result<(), BookParsingError> = x.map_err(Into::into);
    }
}

#[cfg(test)]
pub mod empty_set {
    use error_set::error_set;

    error_set! {
        SetLevelError = {
            EmptySet1,
            EmptySet2,
            MissingDescriptionArg,
        };
        BookParsingError = {
            MissingDescriptionArg,
        };
    }

    #[test]
    fn test() {
        let _empty1 = SetLevelError::EmptySet1;
        let _empty2 = SetLevelError::EmptySet2;
        let book_error = BookParsingError::MissingDescriptionArg;
        let _crate_error_from_book: SetLevelError = book_error.into();
    }
}

#[cfg(test)]
pub mod only_empty_set {
    use error_set::error_set;

    error_set! {
        SetLevelError = {
            EmptySet1,
            EmptySet2,
        };
    }

    #[test]
    fn test() {
        let _empty1 = SetLevelError::EmptySet1;
        let _empty2 = SetLevelError::EmptySet2;
    }
}

#[cfg(test)]
pub mod error_sources_of_same_name {
    use error_set::error_set;

    error_set! {
        SetLevelError = {
            IoError(std::io::Error),
        };
        X = {
            IoError(std::io::Error),
        };
        Y = {
            IoError(std::io::Error),
        };
    }

    #[test]
    fn test() {
        let x = X::IoError(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            "oops out of memory",
        ));
        let y: Y = x.into();
        let _set: SetLevelError = y.into();
    }
}

#[cfg(test)]
pub mod error_sources_of_different_names {
    use error_set::error_set;

    error_set! {
        SetLevelError = {
            IoError(std::io::Error),
        };
        X = {
            IoError(std::io::Error),
        };
        Y = {
            IoError2(std::io::Error),
        };
    }

    #[test]
    fn test() {
        let x = X::IoError(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            "oops out of memory",
        ));
        let y: Y = x.into();
        assert!(matches!(y, Y::IoError2(_)));
        let _set: SetLevelError = y.into();
    }
}

#[cfg(test)]
pub mod readme_example {
    use error_set::error_set;

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

    #[test]
    fn test() {
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
}

#[cfg(test)]
pub mod readme_example_aggregation {
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

    #[test]
    fn test() {
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
}
#[cfg(test)]
pub mod should_not_compile_tests {

    #[test]
    fn multiple_same_sources() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/multiple_same_sources.rs");
    }

    #[test]
    fn two_enums_same_name() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/two_enums_same_name.rs");
    }

    #[test]
    fn recursive_dependency() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/recursive_dependency.rs");
    }
}