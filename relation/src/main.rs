mod db;
mod graphql;

mod prelude {
    pub use crate::db::*;
    pub use crate::graphql::*;

    pub use grand_line::*;
    pub use sea_orm::prelude::*;
    pub use sea_orm::*;
}

use crate::prelude::*;
use axum::{Router, routing::get_service, serve};
use grand_line::async_graphql_axum::GraphQL;
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::fmt::Subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: add feature flag tracing
    Subscriber::builder().with_max_level(Level::DEBUG).init();

    let db = Arc::new(init_db().await?);
    let schema = init_schema(db);
    let svc = GraphQL::new(schema);
    let router = get_service(svc.clone()).post_service(svc);
    let app = Router::new().route("/api/graphql", router);
    // TODO: catch panic unwind using axum layer

    let port = 4000;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await?;

    println!("listening on port {}", port);
    serve(listener, app).await?;

    Ok(())
}
