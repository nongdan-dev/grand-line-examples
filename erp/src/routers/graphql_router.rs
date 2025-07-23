use std::sync::Arc;

use crate::context::auth_context::{AuthContext, AuthContextTrait};
use crate::schema::{init_schema, MySchema};

use grand_line::async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use grand_line::axum::extract::State;
use grand_line::axum::http::HeaderMap;
use grand_line::axum::routing::post;
use grand_line::axum::Router;
use grand_line::sea_orm::DatabaseConnection;
use grand_line::*;
#[derive(Clone)]
pub struct AppState {
    pub schema: MySchema,
    pub db: Arc<DatabaseConnection>,
}
async fn graphql_handler(
    State(app): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let auth_context = AuthContext::new(headers, app.db);
    app.schema
        .execute(req.into_inner().data(auth_context))
        .await
        .into()
}

pub fn graphql_router(db: &sea_orm::DatabaseConnection) -> Router {
    let arc = Arc::new(db.clone());
    let schema = init_schema(db.clone());
    Router::new().route(
        "/api/graphql",
        post(graphql_handler).with_state(AppState { schema, db:arc }),
    )
}
