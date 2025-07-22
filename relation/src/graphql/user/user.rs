use grand_line::*;

#[model(no_by_id = true, no_deleted_at = true)]
pub struct User {
    pub name: String,
}
