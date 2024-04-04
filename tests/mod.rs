#[cfg(test)]
pub mod regular {
    use error_set::error_set;

    error_set!(
        SetLevelError {
            MagazineParsingError {
                MissingNameArg,
            },
            BookParsingError {
                MissingNameArg,
                MissingPublishTimeArg,
                MissingDescriptionArg,
            },
        }
    );

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

    error_set!(
        SetLevelError {
            EmptySet1,
            BookParsingError {
                MissingDescriptionArg,
            },
            EmptySet2,
        }
    );

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

    error_set!(SetLevelError {
        EmptySet1,
        EmptySet2,
    });

    #[test]
    fn test() {
        let _empty1 = SetLevelError::EmptySet1;
        let _empty2 = SetLevelError::EmptySet2;
    }
}

#[cfg(test)]
pub mod error_sources_of_same_name {
    use error_set::error_set;

    error_set!(
        SetLevelError {
            X {
            IoError(std::io::Error),
            },
            Y {
            IoError(std::io::Error),
            },
            IoError(std::io::Error),
        }
    );

    #[test]
    fn test() {
        let x = X::IoError(std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory"));
        let y: Y = x.into();
        let _set: SetLevelError = y.into();
    }
}


#[cfg(test)]
pub mod error_sources_of_different_names {
    use error_set::error_set;

    error_set!(
        SetLevelError {
            X {
            IoError(std::io::Error),
            },
            Y {
            IoError2(std::io::Error),
            },
            IoError(std::io::Error),
        }
    );

    #[test]
    fn test() {
        let x = X::IoError(std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory"));
        let y: Y = x.into();
        assert!(matches!(y, Y::IoError2(_)));
        let _set: SetLevelError = y.into();
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
}

