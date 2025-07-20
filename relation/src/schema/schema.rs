use crate::prelude::*;
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use grand_line::*;
use sea_orm::*;

#[derive(Default, MergedObject)]
pub struct Query(
    TodoSearchQuery,
    // TODO:
);

pub fn init_schema(db: DatabaseConnection) -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(Context::new(db))
        .finish()
}
