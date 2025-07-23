use crate::{
    context::authenticate::{must_anonymous, must_authenticate, try_authenticate, AuthError},
    prelude::*,
};
use grand_line::{async_trait::async_trait, axum::http::HeaderMap, sea_orm::DatabaseConnection, *};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthContext {
    pub db: Arc<DatabaseConnection>,
    pub headers: HeaderMap,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub cache: Arc<tokio::sync::Mutex<Cache>>,
}
#[derive(Default)]
pub struct Cache {
    pub authenticate: Option<LoginSession>,
}
#[async_trait]
#[async_trait]
pub trait AuthContextTrait {
    fn new(headers: HeaderMap,db: Arc<DatabaseConnection>) -> Self;
    async fn must_anonymous(&self) -> Result<(), AuthError>;
    async fn try_authenticate(&self) -> Result<LoginSession,AuthError> ;
    async fn must_authenticate(&self) -> Result<LoginSession,AuthError> ;
}
#[async_trait]

impl AuthContextTrait for AuthContext {
    fn new(headers: HeaderMap,db:Arc<DatabaseConnection> ) -> Self {
        let ip_address = headers
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        let user_agent = headers
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        Self {
            headers,
            ip_address,
            user_agent,
            cache: Arc::new(tokio::sync::Mutex::new(Cache::default())),
            db
        }
    }
    async fn must_anonymous(&self) -> Result<(), AuthError> {
        must_anonymous(self).await
    }
    async fn try_authenticate(&self) ->Result<LoginSession,AuthError> {
        try_authenticate(self).await
    }
    async fn must_authenticate(&self) -> Result<LoginSession, AuthError> {
        must_authenticate(self).await
    }
}
