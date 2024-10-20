use crate::prelude::*;
use grand_line::*;

#[todo_update]
fn resolver() {
    let content = data.content.unwrap_or_default();
    println!("todoUpdate id={} data={}", id, &content);

    let todo = todo_db_detail(&tx, id).await?;
    let am = todo_active_update!({
        content,
        ..todo.into()
    });
    am.update(&tx).await?
}
