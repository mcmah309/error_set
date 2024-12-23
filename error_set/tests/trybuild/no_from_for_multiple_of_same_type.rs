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

fn main() {
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