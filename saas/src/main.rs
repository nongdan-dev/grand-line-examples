mod schema;

mod prelude {
    pub use crate::schema::*;
    pub use models::*;
}

use crate::prelude::*;
use async_graphql::http::{AltairConfigOptions, AltairSource, AltairWindowOptions};
use async_graphql_axum::GraphQL;
use axum::{
    extract::Request,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    serve, Router, ServiceExt,
};
use grand_line::*;
use sea_orm::{prelude::*, *};
use std::error::Error;
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = build_schema().data(init_db().await?).finish();

    let app = Router::new().route("/api/altair", get(altair)).route(
        "/api/graphql",
        get_service(GraphQL::new(schema.clone())).post_service(GraphQL::new(schema)),
    );

    let app = NormalizePathLayer::trim_trailing_slash().layer(app);
    let app = ServiceExt::<Request>::into_make_service(app);

    let port = 4000;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await?;

    println!("listening on port {}", port);
    serve(listener, app).await?;

    Ok(())
}

async fn altair() -> impl IntoResponse {
    Html(
        AltairSource::build()
            .options(AltairConfigOptions {
                window_options: Some(AltairWindowOptions {
                    endpoint_url: Some("/api/graphql".to_owned()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .finish(),
    )
}

async fn init_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db = Database::connect("sqlite::memory:").await?;

    db.execute_unprepared(
        "CREATE TABLE todo (
            id TEXT PRIMARY KEY NOT NULL
            , content TEXT NOT NULL
            , done INT(1) NOT NULL
            , created_at TEXT NOT NULL
            , updated_at TEXT
        );",
    )
    .await?;

    TodoEntity::insert_many(vec![
        todo_active_create!({
            content: "2023 good bye",
            done: true,
        }),
        todo_active_create!({
            content: "2023 great",
            done: true,
        }),
        todo_active_create!({
            content: "2024 hello",
            done: false,
        }),
        todo_active_create!({
            content: "2024 awesome",
            done: false,
        }),
    ])
    .exec(&db)
    .await?;

    Ok(db)
}
