use grand_line::sea_orm::EntityTrait;

use crate::libs::helpers::auth_qs::{qs_id_secret_parse, IdSecret};
use crate::prelude::*;
use crate::{
    config::{LOGIN_SESSION_COOKIE_KEY, LOGIN_SESSION_HEADER_PREFIX},
    context::auth_context::AuthContext,
    libs::utils::cookie::parse_cookie,
};
use std::sync::Arc;
#[derive(Debug)]
pub enum AuthError {
    AlreadyLoggedIn,
    Unauthenticated,
}
pub async fn must_anonymous(ctx: &AuthContext) -> Result<(), AuthError> {
    // match try_authenticate(ctx).await {
    //     Some(_) => Err(AuthError::AlreadyLoggedIn),
    //     None => Ok(()),
    // }
     Err(AuthError::AlreadyLoggedIn)
}
pub async fn must_authenticate(ctx: &AuthContext) -> Result<LoginSession, AuthError> {
    try_authenticate(ctx)
        .await
        
}
pub async fn try_authenticate(ctx: &AuthContext) -> Result<LoginSession,AuthError> {
    // let mut cache = ctx.cache.lock().await;
    // if let Some(ref session) = cache.authenticate {
    //     return Some(Arc::new(session.clone()));
    // }
    // let session = try_authenticate_without_cache(ctx).await;
    // cache.authenticate = session.clone();
    try_authenticate_without_cache(ctx);
    Err(AuthError::AlreadyLoggedIn)
}

async fn try_authenticate_without_cache(ctx: &AuthContext) -> Result<LoginSessionSql, AuthError> {
    let db = &ctx.db;
    let token = get_login_token(ctx);
    match token {
        Some(t) => {
            let s = LoginSession::find_by_id(t.id).one(db.as_ref()).await;
            match s {
                Ok(s) => match s {
                    Some(s) => {
                        if s.secret != t.secret {
                            return Err(AuthError::Unauthenticated);
                        } else {
                            return Ok(s);
                        }
                    }
                    None => return Err(AuthError::Unauthenticated),
                },
                Err(_) => return Err(AuthError::Unauthenticated),
            }
        }
        None =>  return Err(AuthError::Unauthenticated)
    }
   
}

fn get_login_token(ctx: &AuthContext) -> Option<IdSecret> {
    if let Some(cookie_header) = ctx.headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            let cookies = parse_cookie(cookie_str);
            if let Some(session_cookie) = cookies.get(LOGIN_SESSION_COOKIE_KEY) {
                if let Some(token) = qs_id_secret_parse(Some(session_cookie)) {
                    return Some(token);
                }
            }
        }
    }
    if let Some(auth_head) = ctx.headers.get("authorization") {
        if let Ok(auth_head_str) = auth_head.to_str() {
            if auth_head_str.starts_with(LOGIN_SESSION_HEADER_PREFIX) {
                let token_part = auth_head_str.replace(LOGIN_SESSION_HEADER_PREFIX, "");
                if let Some(token) = qs_id_secret_parse(Some(&token_part)) {
                    return Some(token);
                }
            }
        }
    }

    None
}
