use crate::prelude::*;
use grand_line::*;
use sea_orm::entity::*;
#[model]
pub struct Industry {
    pub name: String,
    pub description: String,
    pub business_type: String,
}
