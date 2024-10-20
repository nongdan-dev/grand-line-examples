use crate::prelude::*;
use grand_line::*;

#[todo_search]
fn todoSearch2024() {
    let f = todo_filter_some!({
        content_starts_with: "2024",
    });
    let o = todo_order_by_some!([ContentAsc]);
    (f, o)
}
