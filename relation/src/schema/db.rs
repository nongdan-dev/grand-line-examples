use crate::prelude::*;
use grand_line::*;
use sea_orm::*;
use std::error::Error;

pub async fn init_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = Database::connect("sqlite::memory:").await?;

    db.execute_unprepared(
        "CREATE TABLE user (
            id TEXT PRIMARY KEY NOT NULL
            , name TEXT NOT NULL
            , created_at TEXT NOT NULL
            , updated_at TEXT
        );",
    )
    .await?;

    User::insert_many(vec![
        active_create!(User { name: "Olivia" }),
        active_create!(User { name: "Peter" }),
    ])
    .exec(&db)
    .await?;
    let users = User::find().all(&db).await?;
    let user_id0 = users.get(0).map(|u| u.id.clone());
    let user_id1 = users.get(1).map(|u| u.id.clone());

    db.execute_unprepared(
        "CREATE TABLE todo (
            id TEXT PRIMARY KEY NOT NULL
            , content TEXT NOT NULL
            , done INT(1) NOT NULL
            , created_at TEXT NOT NULL
            , created_by_id TEXT
            , updated_at TEXT
            , updated_by_id TEXT
        );",
    )
    .await?;

    Todo::insert_many(vec![
        active_create!(Todo {
            content: "2023 good bye",
            done: true,
            created_by_id: user_id0.clone(),
        }),
        active_create!(Todo {
            content: "2023 great",
            done: true,
            created_by_id: user_id0.clone(),
        }),
        active_create!(Todo {
            content: "2024 hello",
            done: false,
            created_by_id: user_id1.clone(),
        }),
        active_create!(Todo {
            content: "2024 awesome",
            done: false,
            created_by_id: user_id1.clone(),
        }),
    ])
    .exec(&db)
    .await?;

    Ok(db)
}
