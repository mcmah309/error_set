use error_set::error_set;

error_set!(
    SetLevelError {
        X {
        IoError2,
        },
        X {
        IoError,
        },
    }
);

pub fn main() {}