use crate::{
    context::auth_context::{AuthContext, AuthContextTrait},
    prelude::*,
};
use grand_line::*;

#[create(LoginSession)]
fn resolver() {
    let auth_ctx = ctx.data_opt::<AuthContext>().unwrap();
    auth_ctx.try_authenticate().await.unwrap();
    let am = active_create!(LoginSession {
        
    });
    am.insert(tx).await?.into()
    // TODO:
}
