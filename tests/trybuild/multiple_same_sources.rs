use error_set::error_set;

error_set! {
    X {
        IoError(std::io::Error),
        IoError2(std::io::Error),
    },
    Y {
        IoError2(std::io::Error),
    },
}

pub fn main() {}
