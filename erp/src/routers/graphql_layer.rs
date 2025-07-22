use crate::context::auth_context::{AuthContext, AuthContextTrait};
use grand_line::axum::{extract::Request, middleware::Next, response::Response};

pub async fn graphql_layer(mut req: Request, next: Next) -> Response {
    let headers = req.headers().clone();
    let context = AuthContext::new(headers);
    req.extensions_mut().insert(context);

    next.run(req).await
}
