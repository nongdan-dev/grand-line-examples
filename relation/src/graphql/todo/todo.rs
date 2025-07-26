use crate::prelude::*;
use grand_line::*;

#[model]
pub struct Todo {
    pub content: String,
    pub done: bool,
    #[belongs_to]
    pub created_by: User,
}
