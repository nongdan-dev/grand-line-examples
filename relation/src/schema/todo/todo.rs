use crate::prelude::*;
use grand_line::*;

#[model(no_deleted_at = true)]
pub struct Todo {
    pub content: String,
    pub done: bool,
    #[belongs_to]
    pub created_by: User,
}
