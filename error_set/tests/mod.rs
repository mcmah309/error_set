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
pub mod multiple_error_sources_of_same_type {
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
        LoginError<T: Debug> = {
            IoError(std::io::Error),
        } || AuthError1<T> || AuthError2<T>;

        X<G: Debug> = {
            A {
                a: G
            }
        };
        Y<H: Debug> = {
            B {
                b: H
            }
        };
        Z<T: Debug> = X<T> || Y<T>;
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

        let _x: X<i32> = X::A { a: 1 };
        //let z: Z<i32> = x.into();
        //matches!(z, Z::A { a: _ });

        let _y: Y<i32> = Y::B { b: 1 };
        //let z: Z<i32> = y.into();
        //matches!(z, Z::B { b: _ });
    }
}

#[cfg(test)]
pub mod disable {
    use std::{
        error::Error,
        fmt::{Debug, Display},
    };

    use error_set::error_set;

    error_set! {
        U = {
            IoError(std::io::Error),
        };
        V = {
            FmtError(std::fmt::Error),
            IoError(std::io::Error),
        };
        #[disable(From(std::io::Error,U))]
        W = V || U;
        #[disable(From)]
        X = {
            A
        };
        #[disable(Display,Error)]
        Y = {
            A,
        };
        #[disable(Debug)]
        Z = {
            A,
        };
    }

    impl From<U> for W {
        fn from(u: U) -> Self {
            match u {
                U::IoError(e) => W::IoError(e),
            }
        }
    }

    impl From<std::io::Error> for W {
        fn from(e: std::io::Error) -> Self {
            W::IoError(e)
        }
    }

    impl From<Y> for X {
        fn from(x: Y) -> Self {
            match x {
                Y::A => X::A,
            }
        }
    }

    impl Display for Y {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Y")
        }
    }

    impl Error for Y {}

    impl Debug for Z {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Z")
        }
    }

    #[test]
    fn test() {
        let u: U = std::io::Error::new(std::io::ErrorKind::Other, "oops").into();
        let w: W = u.into();
        assert!(matches!(w, W::IoError(_)));
        let w: W = std::io::Error::new(std::io::ErrorKind::Other, "oops").into();
        assert!(matches!(w, W::IoError(_)));
        let v = V::IoError(std::io::Error::new(std::io::ErrorKind::Other, "oops"));
        let w: W = v.into();
        assert!(matches!(w, W::IoError(_)));

        let x = X::A;
        let y: Y = x.into();
        assert!(matches!(y, Y::A));

        let y = Y::A;
        let x: X = y.into();
        assert!(matches!(x, X::A));

        let y = Y::A;
        assert_eq!(format!("{}", y), "Y");

        let err: Box<dyn Error> = y.into();
        assert_eq!(err.to_string(), "Y");

        let z = Z::A;
        assert_eq!(format!("{:?}", z), "Z");
    }
}

#[cfg(test)]
pub mod from_for_generic_and_regular {
    use error_set::error_set;

    error_set! {
        #[disable(From(E))]
        X<E: core::error::Error + core::fmt::Debug + core::fmt::Display> = Y || Z<E>;
        Y = {
            A,
        };
        Z<E: core::error::Error + core::fmt::Debug + core::fmt::Display> = {
            B(E),
        };
    }

    #[test]
    fn test() {
        let y = Y::A;
        let x: X<std::io::Error> = y.into();
        assert!(matches!(x, X::A));

        let z = Z::B(std::io::Error::new(std::io::ErrorKind::Other, "oops"));
        let x: X<std::io::Error> = z.into();
        assert!(matches!(x, X::B(_)));
    }
}

#[cfg(test)]
pub mod generics_nested {
    use error_set::error_set;

    #[derive(Debug)]
    pub struct Wrapper<T: core::fmt::Debug + core::fmt::Display>(T);

    impl<T: core::fmt::Debug + core::fmt::Display> std::fmt::Display for Wrapper<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Wrapper({})", self.0)
        }
    }

    error_set!{
        X<H: core::fmt::Debug + core::fmt::Display> = {
            A {
                a: Wrapper<H>
            }
        };
        Z<T: core::fmt::Debug + core::fmt::Display> = X<T>;
    }

    #[test]
    fn test() {
        let _x = X::A { a: Wrapper(1) };
        let _z = Z::A { a: Wrapper(1) };
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
    fn floating_attributes() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/floating_attributes.rs");
    }

    #[test]
    fn generic_specification_needed() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/generic_specification_needed.rs");
    }

    #[test]
    fn no_from_for_multiple_of_same_type() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/trybuild/no_from_for_multiple_of_same_type.rs");
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
