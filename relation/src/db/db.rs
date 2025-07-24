use crate::prelude::*;

pub async fn init_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = Database::connect("sqlite::memory:").await?;

    db.execute_unprepared(
        "CREATE TABLE user (
            id TEXT PRIMARY KEY NOT NULL
            , name TEXT NOT NULL
            , email TEXT NOT NULL
            , hashed_password TEXT NOT NULL
            , created_at TEXT NOT NULL
            , updated_at TEXT
        );",
    )
    .await?;

    User::insert_many(vec![
        active_create!(User {
            name: "Olivia",
            email: "olivia@fringe.org",
            hashed_password: "",
        }),
        active_create!(User {
            name: "Peter",
            email: "peter@fringe.org",
            hashed_password: "",
        }),
    ])
    .exec(&db)
    .await?;
    let users = User::find().all(&db).await?;
    let user_id0 = users.get(0).map(|u| u.id.clone());
    let user_id1 = users.get(1).map(|u| u.id.clone());

    db.execute_unprepared(
        "CREATE TABLE org (
            id TEXT PRIMARY KEY NOT NULL
            , name TEXT NOT NULL
            , created_at TEXT NOT NULL
            , updated_at TEXT
        );",
    )
    .await?;

    Org::insert_many(vec![active_create!(Org { name: "Fringe" })])
        .exec(&db)
        .await?;
    let orgs = Org::find().all(&db).await?;
    let org_id0 = orgs.get(0).map(|o| o.id.clone());

    db.execute_unprepared(
        "CREATE TABLE user_in_org (
            id TEXT PRIMARY KEY NOT NULL
            , user_id TEXT NOT NULL
            , org_id TEXT NOT NULL
            , created_at TEXT NOT NULL
            , updated_at TEXT
        );",
    )
    .await?;
    UserInOrg::insert_many(vec![
        active_create!(UserInOrg {
            user_id: user_id0.as_ref().unwrap().to_string(),
            org_id: org_id0.as_ref().unwrap().to_string(),
        }),
        active_create!(UserInOrg {
            user_id: user_id1.as_ref().unwrap().to_string(),
            org_id: org_id0.as_ref().unwrap().to_string(),
        }),
    ])
    .exec(&db)
    .await?;

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
