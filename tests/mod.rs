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
    use error_set::{error_set, CoerceResult};

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
            CouldNotReadBook(std::io::Error),
            MissingName,
            NoContents,
        };
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

    #[test]
    fn test() {
        let book_section_parsing_error: BookSectionParsingError =
            BookSectionParsingError::MissingName;
        let book_parsing_error: BookParsingError = book_section_parsing_error.into();
        assert!(matches!(book_parsing_error, BookParsingError::MissingName));
        let media_error: MediaError = book_parsing_error.into();
        assert!(matches!(media_error, MediaError::MissingName));

        let io_error = std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory");
        let result_download_error: Result<(), DownloadError> = Err(io_error).coerce();
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

    #[test]
    fn test() {
        let book_section_parsing_error = BookSectionParsingError::MissingName;
        let book_parsing_error: BookParsingError = book_section_parsing_error.into();
        assert!(matches!(book_parsing_error, BookParsingError::MissingName));
        let media_error: MediaError = book_parsing_error.into();
        assert!(matches!(media_error, MediaError::MissingName));

        let io_error = std::io::Error::new(std::io::ErrorKind::OutOfMemory, "oops out of memory");
        let result_download_error: Result<(), DownloadError> = Err(io_error).map_err(Into::into);
        let result_media_error: Result<(), MediaError> = result_download_error.map_err(Into::into);
        assert!(matches!(result_media_error, Err(MediaError::IoError(_))));
    }
}

#[cfg(feature = "coerce_macro")]
#[cfg(test)]
pub mod coerce_macro {
    use error_set::error_set;

    error_set! {
        SetX = {
            X
        } || Common;
        #[derive(PartialEq,Eq)]
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

    fn setx_result() -> Result<(), SetX> {
        Err(SetX::A)
    }
    fn setx() -> SetX {
        SetX::A
    }

    fn setx_result_to_sety_result_coerce_return() -> Result<(), SetY> {
        let _ok = coerce!(setx_result() => {
            Ok(ok) => ok,
            Err(SetX::X) => (), // handle
            { Err(SetX) => return Err(SetY) }
        });
        Ok(())
    }
    fn setx_result_to_sety_result_coerce() -> Result<(), SetY> {
        let result: Result<(), SetY> = coerce!(setx_result() => {
            Ok(_) => Err(SetY::D),
            Err(SetX::X) => Err(SetY::F), // handle
            { Err(SetX) => Err(SetY) }
        });
        result
    }
    fn setx_to_sety_coerce() -> SetY {
        let sety = coerce!(setx() => {
            SetX::X => SetY::C, // handle
            {SetX => SetY}
        });
        sety
    }
    fn setx_to_sety_coerce_return() -> SetY {
        let sety = coerce!(setx() => {
            SetX::X => SetY::G, // handle
            {SetX => return SetY}
        });
        sety
    }

    fn setx_result_to_sety_result() -> Result<(), SetY> {
        let _ok = match setx_result() {
            Ok(ok) => ok,
            Err(SetX::X) => {}
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

    #[test]
    fn test() {
        assert_eq!(
            setx_result_to_sety_result_coerce_return().unwrap_err(),
            SetY::A
        );
        assert_eq!(setx_result_to_sety_result_coerce().unwrap_err(), SetY::A);
        assert_eq!(setx_to_sety_coerce(), SetY::A);
        assert_eq!(setx_to_sety_coerce_return(), SetY::A);

        assert_eq!(setx_result_to_sety_result().unwrap_err(), SetY::A);
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
        assert!(matches!(result_media_error, Err(MediaError::IoError(_))));
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

#[cfg(feature = "tracing")]
#[cfg(test)]
mod tracing {
    use error_set::RecordErr;
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
}

#[cfg(feature = "log")]
#[cfg(test)]
mod log {
    use error_set::RecordErr;
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
