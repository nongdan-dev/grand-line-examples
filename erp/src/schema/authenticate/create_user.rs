use crate::{libs::utils::hash_password::hash_password, prelude::*};
use grand_line::*;

#[create(User)]
async fn resolver() {
    let email_lower = data.email.0.clone().to_lowercase();
    let email = email_lower.trim().to_string();
    let password = data.password.0.clone();
    let hashed_password = hash_password(&password);
    let am = active_create!(User {
        email,
        hashed_password: hashed_password
    });
    am.insert(tx).await?.into()
}
