use crate::config::LOGIN_SESSION_EXPIRES;
use crate::libs::helpers::auth_qs::{qs_id_secret, qs_id_secret_parse, IdSecret};
use crate::prelude::*;
use crate::{
    config::{LOGIN_SESSION_COOKIE_KEY, LOGIN_SESSION_HEADER_PREFIX},
    context::auth_context::AuthContext,
    libs::utils::cookie::parse_cookie,
};
use cookie::Cookie;
use grand_line::async_graphql::Context;
use grand_line::axum::http::header;
use grand_line::chrono::Utc;
use grand_line::sea_orm::ActiveValue::Set;
use grand_line::sea_orm::{ActiveModelTrait, EntityTrait, TryIntoModel};
use grand_line::axum::http::HeaderValue;


#[derive(Debug, Clone)]
pub enum AuthError {
    AlreadyLoggedIn,
    Unauthenticated,
    AuthenticationFailed,
}
pub async fn must_anonymous(ctx: &AuthContext) -> Result<(), AuthError> {
    match try_authenticate(ctx).await {
        Ok(_) => Err(AuthError::AlreadyLoggedIn),
        Err(_) => Ok(()),
    }
}
pub async fn must_authenticate(ctx: &AuthContext) -> Result<LoginSessionSql, AuthError> {
    try_authenticate(ctx).await
}
pub async fn try_authenticate(ctx: &AuthContext) -> Result<LoginSessionSql, AuthError> {
    let mut cache = ctx.cache.lock().await;
    if let Some(ref session) = cache.authenticate {
        return Ok(session.clone());
    }
    let session = try_authenticate_without_cache(ctx).await;
    cache.authenticate = session.clone().ok();
    session
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
                        if s.secret != t.secret || is_loin_token_expired(&s) {
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
        None => return Err(AuthError::Unauthenticated),
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
fn is_loin_token_expired(s: &LoginSessionSql) -> bool {
    let now = Utc::now().timestamp_millis();
    let created_at = s.clone().created_at.timestamp_millis();
    now > created_at + LOGIN_SESSION_EXPIRES as i64
}
pub async fn create_login_session(
    ctx: &Context<'_>,
    user_id: &str,
) -> Result<LoginSessionSql, AuthError> {
    let auth_ctx = ctx.data_opt::<AuthContext>().unwrap();
    let active_model = LoginSessionActiveModel {
        user_id: Set(user_id.to_string()),
        ip_address: Set(auth_ctx.ip_address.clone().unwrap_or_default()),
        user_agent: Set(auth_ctx.user_agent.clone().unwrap_or_default()),
        ..Default::default()
    };

    let saved_model = active_model.save(auth_ctx.db.as_ref()).await.map_err(|e| {
        eprintln!("Database error: {:?}", e);
        AuthError::AuthenticationFailed
    })?;

    let model = saved_model.try_into_model().map_err(|e| {
        eprintln!("Model conversion error: {:?}", e);
        AuthError::AuthenticationFailed
    })?;

    set_login_cookie(&ctx, &model);
    Ok(model)
}
pub fn set_login_cookie(ctx: &Context<'_>, s: &LoginSessionSql) {
    let id_secret = IdSecret {
        id: s.clone().id,
        secret: s.clone().secret,
    };
    let cookie_value = match qs_id_secret(&id_secret) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error generating cookie value: {:?}", e);
            return;
        }
    };
    let base_cookie = Cookie::new(LOGIN_SESSION_COOKIE_KEY, cookie_value);
    let cookie = Cookie::build(base_cookie)
        .http_only(true)
        .max_age(cookie::time::Duration::seconds(
            (LOGIN_SESSION_EXPIRES / 1000) as i64,
        ))
        .expires(
            cookie::time::OffsetDateTime::now_utc()
                + cookie::time::Duration::milliseconds(LOGIN_SESSION_EXPIRES as i64),
        )
        .build();
    ctx.append_http_header(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );
}
