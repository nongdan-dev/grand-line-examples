use crate::{
    prelude::*,
};
use async_graphql::{EmptySubscription, MergedObject, Schema};
use grand_line::*;
use sea_orm::*;

#[derive(Default, MergedObject)]
pub struct Query(
    // TODO:
);
pub type MySchema= Schema<Query, Mutation, EmptySubscription>;
#[derive(Default, MergedObject)]
pub struct Mutation(LoginSessionCreateMutation);
pub fn init_schema(db: DatabaseConnection) -> MySchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        // TODO: add tracing extension with feature flag tracing
        .extension(TxExtension)
        .data(GrandLineContext::new(db))
        .finish()
}
