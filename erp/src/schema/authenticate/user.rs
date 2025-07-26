use grand_line::*;

use crate::prelude::{Email, Password};

#[model(no_by_id = true, no_deleted_at = true)]

pub struct User {
    pub email: String,
    pub hashed_password: String,
    // pub person_id: String,
    // pub org_id: String,
}
#[input]
pub struct UserCreate {
    pub email: Email,
    pub password: Password,
}