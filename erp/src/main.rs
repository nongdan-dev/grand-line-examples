mod context;
mod custom_scalars;
mod routers;
mod schema;
mod config;
mod libs;
mod prelude {
    pub use crate::custom_scalars::*;
    pub use crate::schema::*;
}
use routers::graphql_router;
use crate::prelude::*;
use axum::serve;

use grand_line::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db = init_db().await?;
    let app = graphql_router(&db);
    let port = 4000;
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await?;

    println!("listening on port {}", port);
    serve(listener, app).await?;

    Ok(())
}
