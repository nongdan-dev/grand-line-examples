use grand_line::*;

#[model]

pub struct User {
    pub email: String,
    pub hashed_password: String,
    pub person_id: String,
    pub org_id: String,
}
