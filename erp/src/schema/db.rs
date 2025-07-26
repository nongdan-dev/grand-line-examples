use crate::prelude::*;
use grand_line::*;
use sea_orm::*;

pub async fn init_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = Database::connect("sqlite::memory:").await?;
    db.execute_unprepared(
        "CREATE TABLE user (
            id TEXT PRIMARY KEY NOT NULL
            , email TEXT NOT NULL
            , hashed_password INT(1) NOT NULL
            , created_at TEXT NOT NULL
            , updated_at TEXT
        );",
    )
    .await?;
    
    Ok(db)
}
