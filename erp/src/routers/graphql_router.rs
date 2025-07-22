

use grand_line::axum::routing::get_service;
use grand_line::axum::Router;
use grand_line::*;
use async_graphql_axum::GraphQL;
use crate::routers::graphql_layer::graphql_layer;
use crate::schema::init_schema;

pub fn graphql_router(db: &sea_orm::DatabaseConnection) -> Router {
    let schema = init_schema(db.clone());
    Router::new()
        .route(
            "/api/graphql",
            get_service(GraphQL::new(schema.clone())).post_service(GraphQL::new(schema)),
        )
        .layer(axum::middleware::from_fn(graphql_layer)) 
}