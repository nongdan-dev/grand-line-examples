use crate::prelude::*;
use grand_line::*;

#[model]
pub struct UserInOrg {
    pub user_id: String,
    pub org_id: String,
}
