use crate::prelude::*;
use grand_line::*;

#[todo_create]
fn resolver() {
    let content = data.content;
    println!("todoCreate data={}", content);

    let am = todo_active_create!({ content });
    am.insert(&tx).await?
}
