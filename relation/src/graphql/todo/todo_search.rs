use crate::prelude::*;
use grand_line::*;

#[search(Todo)]
fn resolver() {
    (None, None)
}

#[count(Todo)]
fn resolver() {
    None
}
