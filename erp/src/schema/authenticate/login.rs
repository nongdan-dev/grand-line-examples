use crate::{context::auth_context::{AuthContext, AuthContextTrait}, prelude::*};
use grand_line::*;


#[create(LoginSession)]
fn resolver() {
     let auth_ctx = ctx.data::<AuthContext>().unwrap();
     auth_ctx.try_authenticate().await
    // TODO:
    
}
