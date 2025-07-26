use crate::prelude::*;
use grand_line::*;

#[model]
pub struct Org {
    pub name: String,
    #[many_to_many(through = UserInOrg)]
    pub users: User,
}
