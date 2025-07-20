mod schema;
mod prelude {
    pub use crate::schema::*;
}

use crate::prelude::*;
use async_graphql_axum::GraphQL;
use axum::{routing::get_service, serve, Router};
use grand_line::*;
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db = init_db().await?;
    let schema = init_schema(db);

    let app = Router::new().route(
        "/api/graphql",
        get_service(GraphQL::new(schema.clone())).post_service(GraphQL::new(schema)),
    );

    let port = 4000;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await?;

    println!("listening on port {}", port);
    serve(listener, app).await?;

    Ok(())
}
