use crate::{
    context::auth_context::{AuthContext, AuthContextTrait},
    prelude::*,
};
use grand_line::{async_graphql::{Error, InputType}, *};

#[create(LoginSession)]
fn resolver() {
    let auth_ctx = ctx.data_opt::<AuthContext>().unwrap();
    auth_ctx.must_anonymous().await;
    let email = data.email.0.clone().to_lowercase().trim();
    let password = data.password.0.clone();
    let u = 
    // TODO:
}
