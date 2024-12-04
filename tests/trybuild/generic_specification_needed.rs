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
    } || AuthError1 || AuthError2 || AuthError3;
}

pub fn main() {}