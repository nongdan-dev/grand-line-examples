use crate::prelude::*;
use grand_line::*;

#[model]
pub struct User {
    pub name: String,
    pub email: String,
    #[graphql(skip)]
    pub hashed_password: String,
    #[has_one(key = created_by_id)]
    pub todo: Todo,
    #[has_many(key = created_by_id)]
    pub todos: Todo,
}
