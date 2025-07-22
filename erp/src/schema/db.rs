use crate::prelude::*;
use grand_line::*;
use sea_orm::*;

pub async fn init_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = Database::connect("sqlite::memory:").await?;

    Ok(db)
}
