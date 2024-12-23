use error_set::error_set;

error_set! {
    X = {
        IoError(std::io::Error),
    };
    Y = {
        DifferentName(std::io::Error),
    };
    Z = {
        IoError(std::io::Error),
    };
}

fn main() {
    let io_error = std::io::Error::new(
        std::io::ErrorKind::OutOfMemory,
        "oops out of memory",
    );
    let x: X = io_error.into();
    let z: Z = x.into();
    let x: X = z.into();
    let y: Y = x.into();
    let io_error = std::io::Error::new(
        std::io::ErrorKind::OutOfMemory,
        "oops out of memory",
    );
    let y: Y = io_error.into();
}