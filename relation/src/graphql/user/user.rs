use crate::prelude::*;
use grand_line::*;

#[model(no_by_id = true, no_deleted_at = true)]
pub struct User {
    pub name: String,
    #[has_one(key = created_by_id)]
    pub todo: Todo,
    #[has_many(key = created_by_id)]
    pub todos: Todo,
}
