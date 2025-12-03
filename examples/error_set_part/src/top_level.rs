use error_set::error_set_part;

error_set_part! {
    B := {Field2,} || A
}

fn _function() {}

error_set::error_set_part! {
    A := {Field,}
}