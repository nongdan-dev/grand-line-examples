use crate::prelude::*;
use grand_line::*;

#[search(User)]
fn resolver() {
    (None, None)
}

#[count(User)]
fn resolver() {
    None
}
