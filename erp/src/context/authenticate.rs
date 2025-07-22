
use grand_line::axum;

use crate::{config::LOGIN_SESSION_HEADER_PREFIX, context::auth_context::AuthContext};
use std::sync::Arc;
use crate::prelude::*;
#[derive(Debug)]
pub enum AuthError {
    AlreadyLoggedIn,
    Unauthenticated,
}
pub async fn must_anonymous(ctx: &AuthContext) -> Result<(), AuthError> {
    match try_authenticate(ctx).await {
        Some(_) => Err(AuthError::AlreadyLoggedIn),
        None => Ok(()),
    }
}
pub async fn must_authenticate(ctx: &AuthContext) -> Result<Arc<LoginSession>, AuthError> {
    try_authenticate(ctx).await.ok_or(AuthError::Unauthenticated)
}
pub async fn try_authenticate(ctx: &AuthContext) -> Option<Arc<LoginSession>> {
    let mut cache = ctx.cache.lock().await;
    if let Some(ref session) = cache.authenticate {
        return Some(Arc::new(session.clone()));
    }
    let session = try_authenticate_without_cache(ctx).await;
    cache.authenticate = session.clone();
    session
}

async fn try_authenticate_without_cache(ctx: &AuthContext) -> Option<Arc<LoginSession>> {
    let token = get_login_token(ctx);
    if token.is_none() {
        return None;
    }
    let t = token.unwrap();
    let s = find_login_session_by_id(&t.id).await;
    if s.is_none() {
        return None;
    }
    let s = s.unwrap();
    if s.secret != t.secret || is_login_token_expired(&s) {
        return None;
    }
    Some(Arc::new(s))
}

fn get_login_token(ctx: &AuthContext) -> Option<LoginToken> {
    
    None
}