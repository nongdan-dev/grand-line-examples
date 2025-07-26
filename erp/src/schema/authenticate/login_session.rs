use grand_line::*;
use crate::prelude::*;
#[model(no_by_id = true, no_deleted_at = true)]
pub struct LoginSession {
    pub user_id: String,
    pub secret: String,
    pub ip_address: String,
    pub user_agent: String,
}

#[input]
pub struct LoginSessionCreate {
    pub email: Email,
    pub password: Password,
}