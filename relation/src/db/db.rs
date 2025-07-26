use crate::prelude::*;

pub async fn init_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = Database::connect("sqlite::memory:").await?;

    let b = db.get_database_backend();
    let s = sea_orm::Schema::new(b);
    let stmts = vec![
        s.create_table_from_entity(User),
        s.create_table_from_entity(Org),
        s.create_table_from_entity(UserInOrg),
        s.create_table_from_entity(Todo),
    ];
    for s in stmts {
        db.execute(b.build(&s)).await?;
    }

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

    Org::insert_many(vec![active_create!(Org { name: "Fringe" })])
        .exec(&db)
        .await?;
    let orgs = Org::find().all(&db).await?;
    let org_id0 = orgs.get(0).map(|o| o.id.clone());

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
