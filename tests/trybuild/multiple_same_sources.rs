use error_set::error_set;

error_set!(
    SetLevelError {
        X {
        IoError(std::io::Error),
        IoError2(std::io::Error),
        },
        Y {
        IoError2(std::io::Error),
        },
        IoError(std::io::Error),
    }
);

pub fn main() {}