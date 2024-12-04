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
pub mod first_wrapped_type_implements_from_source {
    use error_set::error_set;

    error_set! {
        X = {
            IoError(std::io::Error),
            IoError2(std::io::Error),
        };
        Y = {
            IoError2(std::io::Error),
            IoError(std::io::Error),
        };
        Z = {
            IoError(std::io::Error),
        };
    }

    #[test]
    fn test() {
        let io_error = std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory");
        let z: Z = io_error.into();
        let x: X = z.into();
        matches!(x, X::IoError(_));
        let y: Y = x.into();
        matches!(y, Y::IoError2(_));
        let x: X = y.into();
        matches!(x, X::IoError(_));
        let io_error = std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory");
        let y: Y = io_error.into();
        matches!(y, Y::IoError2(_));
    }
}

#[cfg(test)]
pub mod readme_example {
    use error_set::{error_set, CoerceResult};

    error_set! {
        /// This a doc comment. The syntax below aggregates the referenced errors into the generated enum
        MediaError = DownloadError || BookParsingError;
        /// Since this all of the variants in [DownloadError] are in [MediaError], this can be turned
        /// into a [MediaError] with just `.into()` or `?`. Note restating variants directly,
        /// instead of using `||`, also works
        DownloadError = {
            InvalidUrl,
            /// The `From` trait for `std::io::Error` will also be automatically derived
            IoError(std::io::Error),
        };
        /// Traits like `Debug`, `Display`, `Error`, and `From` are all automatically derived,
        /// but one can always add more like below
        #[derive(Clone)]
        BookParsingError = {
            #[display("Easily add custom display messages that work just like the `format!` macro {}", i32::MAX)]
            MissingBookDescription,
        } || BookSectionParsingError; // Note the aggregation here as well
        BookSectionParsingError = {
            /// Inline structs are also supported
            #[display("Display messages can also reference fields, like {field}")]
            MissingField {
                field: String
            },
            NoContent,
        };
    }

    #[test]
    fn test() {
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
        let result_download_error: Result<(), DownloadError> = Err(io_error).coerce();
        let result_media_error: Result<(), MediaError> = result_download_error.map_err(Into::into);
        assert!(matches!(result_media_error, Err(MediaError::IoError(_))));
    }
}

#[cfg(test)]
pub mod readme_example_full_expansion {
    use error_set::{error_set, CoerceResult};

    error_set! {
        /// This a doc comment. The syntax below aggregates the referenced errors into the generated enum
        MediaError = {
            InvalidUrl,
            /// The `From` trait for `std::io::Error` will also be automatically derived
            IoError(std::io::Error),
            #[display("Easily add custom display messages that work just like the `format!` macro {}", i32::MAX)]
            MissingBookDescription,
            #[display("Display messages can also reference fields, like {field}")]
            MissingField {
                field: String
            },
            NoContent,
        };
        /// Since this all of the variants in [DownloadError] are in [MediaError], this can be turned
        /// into a [MediaError] with just `.into()` or `?`. Note restating variants directly,
        /// instead of using `||`, also works
        DownloadError = {
            InvalidUrl,
            /// The `From` trait for `std::io::Error` will also be automatically derived
            IoError(std::io::Error),
        };
        /// Traits like `Debug`, `Display`, `Error`, and `From` are all automatically derived,
        /// but one can always add more like below
        #[derive(Clone)]
        BookParsingError = {
            #[display("Easily add custom display messages that work just like the `format!` macro {}", i32::MAX)]
            MissingBookDescription,
            /// Inline structs are also supported
            #[display("Display messages can also reference fields, like {field}")]
            MissingField {
                field: String
            },
            NoContent,
        };
        BookSectionParsingError = {
            /// Inline structs are also supported
            #[display("Display messages can also reference fields, like {field}")]
            MissingField {
                field: String
            },
            NoContent,
        };
    }

    #[test]
    fn test() {
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
        let result_download_error: Result<(), DownloadError> = Err(io_error).coerce();
        let result_media_error: Result<(), MediaError> = result_download_error.map_err(Into::into);
        assert!(matches!(result_media_error, Err(MediaError::IoError(_))));
    }
}

#[cfg(test)]
pub mod documentation {
    use error_set::{error_set, CoerceResult};

    error_set! {
        /// This is a MediaError doc
        MediaError = {
            /// This is a variant IoError doc
            IoError(std::io::Error)
            } || BookParsingError || DownloadError || UploadError;
        /// This is a BookParsingError doc
        BookParsingError = {
            /// This is a variant MissingDescriptionArg doc
            MissingDescriptionArg
        } || BookSectionParsingError;
        /// This is a BookSectionParsingError doc
        /// on two lines.
        #[derive(Clone)]
        BookSectionParsingError = {
            /// This is a variant MissingNameArg doc
            MissingNameArg,
            /// This is a variant NoContents doc
            /// on two lines.
            NoContents,
        };
        /// This is a DownloadError doc
        DownloadError = {
            /// This is a variant CouldNotConnect doc
            CouldNotConnect,
            /// This is a variant OutOfMemory doc
            OutOfMemory(std::io::Error),
        };
        /// This is a UploadError doc
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

        let io_error = std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory");
        let result_download_error: Result<(), DownloadError> = Err(io_error).coerce();
        let result_media_error: Result<(), MediaError> = result_download_error.coerce();
        assert!(matches!(
            result_media_error,
            Err(MediaError::OutOfMemory(_))
        ));
    }
}

#[cfg(test)]
pub mod value_variants1 {
    use error_set::error_set;

    error_set! {
        X = {
            IoError(std::io::Error),
            #[display("My name is {}", name)]
            B {
                name: String
            }
        };
        Y = {
            A,
        } || X || Z;
        Z = {
            C {
                val: i32
            },
            #[display("This is some new display")]
            D
        };
        XX = {
            #[display("This message is different {}", name)]
            B {
                name: String
            }
        };
        W = {
            #[display("error `{}` happened because `{}`", error, reason)]
            B {
                error: usize,
                reason: String
            }
        };
    }

    #[test]
    fn test() {
        let x = X::B {
            name: "john".to_string(),
        };
        assert_eq!(x.to_string(), "My name is john".to_string());
        let y: Y = x.into();
        assert_eq!(y.to_string(), "My name is john".to_string());
        let z = Z::D;
        assert_eq!(z.to_string(), "This is some new display".to_string());
        let y: Y = z.into();
        assert_eq!(y.to_string(), "This is some new display".to_string());
        let z = Z::C { val: 1 };
        assert_eq!(z.to_string(), "Z::C".to_string());
        let y: Y = z.into();
        assert_eq!(y.to_string(), "Y::C".to_string());
        let xx = XX::B {
            name: "john".to_string(),
        };
        assert_eq!(xx.to_string(), "This message is different john".to_string());
        let x: X = xx.into();
        assert_eq!(x.to_string(), "My name is john".to_string());
        let w: W = W::B {
            error: 3,
            reason: "oops".to_string(),
        };
        assert_eq!(
            w.to_string(),
            "error `3` happened because `oops`".to_string()
        );
    }
}

#[cfg(test)]
pub mod value_variants2 {
    use error_set::error_set;

    error_set! {
        AuthError = {
            #[display("1 User `{name}` with role}} `{{{role}` does not exist")]
            UserDoesNotExist1 {
                name: String,
                role: u32,
            },
            #[display("2 User `{}` with role}} `{{{}` does not exist", name, role)]
            UserDoesNotExist2 {
                name: String,
                role: u32,
            },
            #[display("The provided credentials are invalid")]
            InvalidCredentials
        };
        LoginError = {
            IoError(std::io::Error),
        } || AuthError;
    }

    #[test]
    fn test() {
        let x: AuthError = AuthError::UserDoesNotExist1 {
            name: "john".to_string(),
            role: 30,
        };
        assert_eq!(
            x.to_string(),
            "1 User `john` with role} `{30` does not exist".to_string()
        );
        let y: LoginError = x.into();
        assert_eq!(
            y.to_string(),
            "1 User `john` with role} `{30` does not exist".to_string()
        );

        let x: AuthError = AuthError::UserDoesNotExist2 {
            name: "john".to_string(),
            role: 30,
        };
        assert_eq!(
            x.to_string(),
            "2 User `john` with role} `{30` does not exist".to_string()
        );
        let y: LoginError = x.into();
        assert_eq!(
            y.to_string(),
            "2 User `john` with role} `{30` does not exist".to_string()
        );

        let x = AuthError::InvalidCredentials;
        assert_eq!(
            x.to_string(),
            "The provided credentials are invalid".to_string()
        );
    }
}

#[cfg(test)]
pub mod display_ref_error {
    use error_set::error_set;

    error_set! {
        X = {
            #[display("X io error")]
            IoError(std::io::Error),
        };
        Y = {
            #[display("Y io error: {}", source)]
            IoError(std::io::Error),
        };
        Y2 = {
            #[display("Y2 io error type: {}", source.kind())]
            IoError(std::io::Error),
        };
        Z = {
            #[display("Z io error: {0}")]
            IoError(std::io::Error),
        };
        YY = {
            #[display("YY io error: {0}", source)]
            IoError(std::io::Error),
        };

        A = {
            #[display(opaque)]
            IoError(std::io::Error),
        };
        B = {
            IoError(std::io::Error),
        };
    }

    #[test]
    fn test() {
        let x = X::IoError(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            "oops out of memory 1",
        ));
        assert_eq!(x.to_string(), "X io error".to_string());

        let y = Y::IoError(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            "oops out of memory 2",
        ));
        assert_eq!(
            y.to_string(),
            "Y io error: oops out of memory 2".to_string()
        );

        let y2 = Y2::IoError(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            "oops out of memory 3",
        ));
        assert_eq!(
            y2.to_string(),
            "Y2 io error type: out of memory".to_string()
        );

        let z = Z::IoError(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            "oops out of memory 4",
        ));
        assert_eq!(
            z.to_string(),
            "Z io error: oops out of memory 4".to_string()
        );

        let yy = YY::IoError(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            "oops out of memory 5",
        ));
        assert_eq!(
            yy.to_string(),
            "YY io error: oops out of memory 5".to_string()
        );

        let y_to_x: X = y.into();
        let x_to_y: Y = x.into();
        assert_eq!(y_to_x.to_string(), "X io error".to_string());
        assert_eq!(
            x_to_y.to_string(),
            "Y io error: oops out of memory 1".to_string()
        );

        let z_to_y2: Y2 = z.into();
        let y2_to_z: Z = y2.into();
        assert_eq!(
            z_to_y2.to_string(),
            "Y2 io error type: out of memory".to_string()
        );
        assert_eq!(
            y2_to_z.to_string(),
            "Z io error: oops out of memory 3".to_string()
        );

        let a = A::IoError(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            "oops out of memory 5",
        ));
        assert_eq!(a.to_string(), "A::IoError".to_string());

        let b = B::IoError(std::io::Error::new(
            std::io::ErrorKind::OutOfMemory,
            "oops out of memory 6",
        ));
        assert_eq!(b.to_string(), "oops out of memory 6".to_string());

        let a_to_b: B = a.into();
        let b_to_a: A = b.into();
        assert_eq!(a_to_b.to_string(), "oops out of memory 5".to_string());
        assert_eq!(b_to_a.to_string(), "A::IoError".to_string());
    }
}

#[cfg(test)]
pub mod fields_with_unique_types {
    use error_set::error_set;

    error_set! {
        AuthError = {
            #[display("User `{name}` with role `{}` does not exist", role.1)]
            UserDoesNotExist1 {
                name: &'static str,
                role: (u32,String),
            },
            UserDoesNotExist2 {
                name: String,
                role: u32,
            },
            #[display("The provided credentials are invalid")]
            InvalidCredentials
        };
        LoginError = {
            IoError(std::io::Error),
        } || AuthError;
    }

    #[test]
    fn test() {
        let x: AuthError = AuthError::UserDoesNotExist1 {
            name: "john",
            role: (30, "1".to_string()),
        };
        assert_eq!(
            x.to_string(),
            "User `john` with role `1` does not exist".to_string()
        );
        let y: LoginError = x.into();
        assert_eq!(
            y.to_string(),
            "User `john` with role `1` does not exist".to_string()
        );
    }
}

#[cfg(test)]
pub mod inline_source_error {
    use error_set::error_set;

    error_set! {
        AuthError = {
            SourceStruct1(std::fmt::Error) {},

            #[display("User `{name}` with role `{}` does not exist", role.1)]
            UserDoesNotExist1(std::io::Error) {
                name: &'static str,
                role: (u32,String),
            },
            UserDoesNotExist2 {
                name: String,
                role: u32,
            },
            #[display("The provided credentials are invalid")]
            InvalidCredentials
        };
        LoginError = {
            IoError(std::io::Error),
            //A
        } || AuthError;
    }

    #[test]
    fn test() {
        let fmt_error = std::fmt::Error::default();
        let auth_error: AuthError = fmt_error.into();
        matches!(auth_error, AuthError::SourceStruct1 { source: _ });
        let login_error: LoginError = auth_error.into();
        matches!(login_error, LoginError::SourceStruct1 { source: _ });
        let fmt_error = std::fmt::Error::default();
        let login_error: LoginError = fmt_error.into();
        matches!(login_error, LoginError::SourceStruct1 { source: _ });
        let auth_error = AuthError::InvalidCredentials;
        let login_error: LoginError = auth_error.into();
        matches!(login_error, LoginError::InvalidCredentials);
    }
}

#[cfg(test)]
pub mod generics {
    use std::fmt::Debug;

    use error_set::error_set;

    error_set! {
        AuthError1<T: Debug> = {
            SourceStruct(std::fmt::Error) {},
            DoesNotExist {
                name: T,
                role: u32,
            },
            InvalidCredentials
        };
        AuthError2<T: Debug> = {
            SourceStruct(std::fmt::Error) {},
            DoesNotExist {
                name: T,
                role: u32,
            },
            InvalidCredentials
        };
        AuthError3<G: Debug> = {
            SourceStruct(std::fmt::Error) {},
            DoesNotExist {
                name: G,
                role: u32,
            },
            InvalidCredentials
        };
        LoginError = {
            IoError(std::io::Error),
        } || AuthError1 || AuthError2;
    }

    #[test]
    fn test() {
        let fmt_error = std::fmt::Error::default();
        let auth_error: AuthError1<i32> = fmt_error.into();
        matches!(auth_error, AuthError1::SourceStruct { source: _ });
        let login_error: LoginError<i32> = auth_error.into();
        matches!(login_error, LoginError::SourceStruct { source: _ });

        let fmt_error = std::fmt::Error::default();
        let auth_error: AuthError2<i32> = fmt_error.into();
        matches!(auth_error, AuthError2::SourceStruct { source: _ });
        let login_error: LoginError<i32> = auth_error.into();
        matches!(login_error, LoginError::SourceStruct { source: _ });

        let fmt_error = std::fmt::Error::default();
        let login_error: LoginError<i32> = fmt_error.into();
        matches!(login_error, LoginError::SourceStruct { source: _ });
        let auth_error = AuthError1::InvalidCredentials;
        let login_error: LoginError<i32> = auth_error.into();
        matches!(login_error, LoginError::InvalidCredentials);
        let auth_error: AuthError2<i32> = AuthError2::InvalidCredentials;
        let login_error: LoginError<i32> = auth_error.into();
        matches!(login_error, LoginError::SourceStruct { source: _ });

        let auth_error: AuthError2<String> = AuthError2::InvalidCredentials;
        let auth_error: AuthError1<String> = auth_error.into();
        matches!(auth_error, AuthError1::InvalidCredentials);
        let auth_error: AuthError2<String> = auth_error.into();
        matches!(auth_error, AuthError2::InvalidCredentials);
    }
}

#[cfg(test)]
pub mod should_not_compile_tests {

    #[test]
    fn depends_on_self() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/depends_on_itself.rs");
    }

    #[test]
    fn error_sources_of_diffrent_names() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/error_sources_of_diffrent_names.rs");
    }

    #[test]
    fn multiple_different_generics() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/multiple_different_generics.rs");
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

#[cfg(feature = "tracing")]
#[cfg(test)]
mod tracing {
    use error_set::{ConsumeDebug, ResultContext};
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_log_error() {
        let result: Result<(), &str> = Err("error");
        let _ = result.error("An error occurred");

        assert!(logs_contain("An error occurred"));
    }

    #[traced_test]
    #[test]
    fn test_log_warn() {
        let result: Result<(), &str> = Err("warning");
        let _ = result.warn("A warning occurred");

        assert!(logs_contain("A warning occurred"));
    }

    #[traced_test]
    #[test]
    fn test_log_info() {
        let result: Result<(), &str> = Err("info");
        let _ = result.info("An info message");

        assert!(logs_contain("An info message"));
    }

    #[traced_test]
    #[test]
    fn test_log_debug() {
        let result: Result<(), &str> = Err("debug");
        let _ = result.debug("A debug message");

        assert!(logs_contain("A debug message"));
    }

    #[traced_test]
    #[test]
    fn test_log_trace() {
        let result: Result<(), &str> = Err("trace");
        let _ = result.trace("A trace message");

        assert!(logs_contain("A trace message"));
    }

    #[traced_test]
    #[test]
    fn test_log_success() {
        let result: Result<(), &str> = Ok(());
        let _ = result.error("This should not log an error");

        assert!(!logs_contain("This should not log an error"));
    }

    //************************************************************************//

    #[traced_test]
    #[test]
    fn test_log_with_error() {
        let result: Result<(), &str> = Err("error");
        let _ = result.with_error(|e| format!("An error occurred `{}`", e));

        assert!(logs_contain("An error occurred `error`"));
    }

    #[traced_test]
    #[test]
    fn test_log_with_warn() {
        let result: Result<(), u32> = Err(10);
        let _ = result.with_warn(|e| format!("A warning occurred `{}`", e));

        assert!(logs_contain("A warning occurred `10`"));
    }

    #[traced_test]
    #[test]
    fn test_log_with_info() {
        let result: Result<(), &str> = Err("info");
        let _ = result.with_info(|e| format!("An info message `{}`", e));

        assert!(logs_contain("An info message `info`"));
    }

    #[traced_test]
    #[test]
    fn test_log_with_debug() {
        let result: Result<(), &str> = Err("debug");
        let _ = result.with_debug(|e| format!("A debug message `{}`", e));

        assert!(logs_contain("A debug message `debug`"));
    }

    #[traced_test]
    #[test]
    fn test_log_with_trace() {
        let result: Result<(), &str> = Err("trace");
        let _ = result.with_trace(|e| format!("A trace message `{}`", e));

        assert!(logs_contain("A trace message `trace`"));
    }

    #[traced_test]
    #[test]
    fn test_log_with_success() {
        let result: Result<(), &str> = Ok(());
        let _ = result.with_error(|_| "This should not log an error");

        assert!(!logs_contain("This should not log an error"));
    }

    //************************************************************************//

    // todo implement consume_with tests

    //************************************************************************//

    #[traced_test]
    #[test]
    fn test_log_consume_error() {
        let result: Result<(), &str> = Err("error consumed");
        let _ = result.consume_error();

        assert!(logs_contain("error consumed"));
    }

    #[traced_test]
    #[test]
    fn test_log_consume_warn() {
        let result: Result<(), &str> = Err("warning consumed");
        let _ = result.consume_warn();

        assert!(logs_contain("warning consumed"));
    }

    #[traced_test]
    #[test]
    fn test_log_consume_info() {
        let result: Result<(), &str> = Err("info consumed");
        let _ = result.consume_info();

        assert!(logs_contain("info consumed"));
    }

    #[traced_test]
    #[test]
    fn test_log_consume_debug() {
        let result: Result<(), &str> = Err("debug consumed");
        let _ = result.consume_debug();

        assert!(logs_contain("debug consumed"));
    }

    #[traced_test]
    #[test]
    fn test_log_consume_trace() {
        let result: Result<(), &str> = Err("trace consumed");
        let _ = result.consume_trace();

        assert!(logs_contain("trace consumed"));
    }
}

#[cfg(feature = "log")]
#[cfg(test)]
mod log {
    use error_set::ResultContext;
    use lazy_static::lazy_static;
    use log::{Level, Metadata, Record};
    use std::sync::{Arc, Mutex};

    struct TestLogger {
        logs: Arc<Mutex<Vec<String>>>,
    }

    impl log::Log for TestLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Trace
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let mut logs = self.logs.lock().unwrap();
                logs.push(format!("{}", record.args()));
            }
        }

        fn flush(&self) {}
    }

    lazy_static! {
        static ref LOGS: Arc<Mutex<Vec<String>>> = {
            let logs = Arc::new(Mutex::new(Vec::new()));
            let test_logger = TestLogger { logs: logs.clone() };

            log::set_boxed_logger(Box::new(test_logger)).unwrap();
            log::set_max_level(log::LevelFilter::Trace);

            logs
        };
    }

    fn logs_contain(expected: &str) -> bool {
        let logs = LOGS.lock().unwrap();
        logs.iter().any(|log| log.contains(expected))
    }

    fn clear_logs() {
        let mut logs = LOGS.lock().unwrap();
        logs.clear();
    }

    #[test]
    fn test_log_error() {
        clear_logs();
        let result: Result<(), &str> = Err("error");
        let _ = result.error("An error occurred");

        assert!(logs_contain("An error occurred"));
    }

    #[test]
    fn test_log_warn() {
        clear_logs();
        let result: Result<(), &str> = Err("warning");
        let _ = result.warn("A warning occurred");

        assert!(logs_contain("A warning occurred"));
    }

    #[test]
    fn test_log_info() {
        clear_logs();
        let result: Result<(), &str> = Err("info");
        let _ = result.info("An info message");

        assert!(logs_contain("An info message"));
    }

    #[test]
    fn test_log_debug() {
        clear_logs();
        let result: Result<(), &str> = Err("debug");
        let _ = result.debug("A debug message");

        assert!(logs_contain("A debug message"));
    }

    #[test]
    fn test_log_trace() {
        clear_logs();
        let result: Result<(), &str> = Err("trace");
        let _ = result.trace("A trace message");

        assert!(logs_contain("A trace message"));
    }

    #[test]
    fn test_log_success() {
        clear_logs();
        let result: Result<(), &str> = Ok(());
        let _ = result.error("This should not log an error");

        assert!(!logs_contain("This should not log an error"));
    }
}
