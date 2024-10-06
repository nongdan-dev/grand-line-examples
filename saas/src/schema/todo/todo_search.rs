use crate::prelude::*;
use grand_line::*;
use serde_json::to_string as json;

#[todo_search]
fn resolver() {
    println!(
        "todoSearch filter={} order_by={} page={}",
        json(&filter)?,
        json(&order_by)?,
        json(&page)?,
    );

    (None, None)
}

#[todo_count]
fn resolver() {
    println!("todoCount filter={}", json(&filter)?);
    None
}
