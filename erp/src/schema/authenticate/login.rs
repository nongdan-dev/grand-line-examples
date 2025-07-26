use crate::{
    context::{
        auth_context::{AuthContext, AuthContextTrait},
        authenticate::create_login_session,
    },
    libs::utils::hash_password::compare_password,
    prelude::*,
    schema::authenticate::user,
};
use anyhow::anyhow;
use grand_line::*;

#[create(LoginSession)]
async fn resolver() {
    let auth_ctx = ctx.data_opt::<AuthContext>().unwrap();
    auth_ctx
        .must_anonymous()
        .await
        .map_err(|e| anyhow!("{:?}", e))?;
    let email_lower = data.email.0.clone().to_lowercase();
    let email = email_lower.trim().to_string();
    let password = data.password.0.clone();
    let u = User::find()
        .filter(user::Column::Email.eq(email.clone()))
        .one(&*auth_ctx.db)
        .await?
        .ok_or_else(|| anyhow!("Email or password is incorrect"))?;
    if !compare_password(Some(password.as_str()), Some(u.hashed_password.as_str())) {
        return Err(anyhow!("Email or password is incorrect").into());
    }
    {
        let s = create_login_session(&ctx, &u.id).await.map_err(|e| anyhow!("{:?}", e))?;
        s.into_active_model()
    }
}
