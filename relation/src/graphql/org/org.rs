use crate::prelude::*;
use grand_line::*;

#[model(no_by_id = true, no_deleted_at = true)]
pub struct Org {
    pub name: String,
    #[many_to_many(through = UserInOrg)]
    pub users: User,
}
