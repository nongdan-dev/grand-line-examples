use crate::prelude::*;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use grand_line::*;
use sea_orm::*;

#[derive(Default, MergedObject)]
pub struct Query(
    UserSearchQuery
);
pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
#[derive(Default, MergedObject)]
pub struct Mutation(LoginSessionCreateMutation, UserCreateMutation);

pub fn init_schema(db: DatabaseConnection) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        // TODO: add tracing extension with feature flag tracing
        .data(Arc::new(db))
        .extension(GrandLineExtension)
        .finish()
}
