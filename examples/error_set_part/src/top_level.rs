use error_set::error_set_part;

error_set_part! {
    B := {Field2,} || A
}

fn function() {}

error_set::error_set_part! {
    A := {Field,}
}