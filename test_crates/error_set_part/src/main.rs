mod error_set;
mod nested;
mod top_level;

use error_set::{A, B, C};

fn operation_a() -> Result<(), A> {
    Err(A::Field)
}

fn operation_b(use_field2: bool) -> Result<String, B> {
    if use_field2 {
        Err(B::Field2)
    } else {
        Err(B::Field)
    }
}

fn operation_c() -> Result<i32, C> {
    Err(C::Field1)
}

fn convert_error_a_to_b() -> Result<(), B> {
    operation_a()?;
    Ok(())
}

fn convert_error_a_to_b_and_b_to_b(use_field2: bool) -> Result<String, B> {
    operation_a()?;

    let result = operation_b(use_field2)?;

    Ok(result)
}

fn main() {
    match operation_a() {
        Ok(_) => panic!("   Success!"),
        Err(A::Field) => {}
    }

    match operation_b(true) {
        Ok(s) => panic!("   Success: {}", s),
        Err(B::Field2) => {}
        Err(B::Field) => panic!("   ✗ Caught error: B::Field (from A)"),
    }

    match operation_c() {
        Ok(n) => panic!("   Success: {}", n),
        Err(C::Field1) => {},
    }

    match convert_error_a_to_b() {
        Ok(_) => panic!("   Success!"),
        Err(_) => {},
    }

    match convert_error_a_to_b_and_b_to_b(false) {
        Ok(s) => panic!("   Success: {}", s),
        Err(_) => {},
    }
}
