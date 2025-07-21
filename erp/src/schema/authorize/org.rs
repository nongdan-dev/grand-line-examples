use grand_line::*;
use sea_orm::entity::*;


#[model]

pub struct Org {
    pub owner_user_id: String,
    pub company_id: String,
    pub industry_id: String,
}
