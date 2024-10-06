use grand_line::*;

#[macro_model]
pub struct Todo {
    pub content: String,
    pub done: bool,
}

#[macro_model]
pub struct User {
    pub email: String,
    pub hashed_password: String,
}
