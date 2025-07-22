use crate::prelude::*;
use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use std::sync::Arc;

#[derive(Default, MergedObject)]
pub struct Query(
    TodoSearchQuery,
    TodoCountQuery,
    // TODO:
);

pub fn init_schema(db: Arc<DatabaseConnection>) -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        // TODO: add tracing extension with feature flag tracing
        .extension(GrandLineExtension)
        .data(db)
        .finish()
}
