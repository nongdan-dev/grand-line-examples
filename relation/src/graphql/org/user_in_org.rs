use crate::prelude::*;
use grand_line::*;

#[model(no_by_id = true, no_deleted_at = true)]
pub struct UserInOrg {
    pub user_id: String,
    pub org_id: String,
}
